// ============================================
// Level 10, Project 3: Binary Protocol
// Learn: Binary parsing, endianness, wire formats, checksums
// ============================================

// ============================================
// Topic 1: Byte Reading
// Learn: Read integers from byte slices, endianness
// ============================================

pub fn read_u8(data: &[u8], offset: usize) -> Option<u8> {
    data.get(offset).copied()
}

pub fn read_u16_be(data: &[u8], offset: usize) -> Option<u16> {
    if offset + 2 > data.len() { return None; }
    Some(u16::from_be_bytes([data[offset], data[offset + 1]]))
}

pub fn read_u16_le(data: &[u8], offset: usize) -> Option<u16> {
    if offset + 2 > data.len() { return None; }
    Some(u16::from_le_bytes([data[offset], data[offset + 1]]))
}

pub fn read_u32_be(data: &[u8], offset: usize) -> Option<u32> {
    if offset + 4 > data.len() { return None; }
    Some(u32::from_be_bytes([data[offset], data[offset+1], data[offset+2], data[offset+3]]))
}

pub fn read_u32_le(data: &[u8], offset: usize) -> Option<u32> {
    if offset + 4 > data.len() { return None; }
    Some(u32::from_le_bytes([data[offset], data[offset+1], data[offset+2], data[offset+3]]))
}

// ============================================
// Topic 2: Byte Writing
// Learn: Write integers to byte buffers
// ============================================

pub fn write_u8(buf: &mut Vec<u8>, value: u8) {
    buf.push(value);
}

pub fn write_u16_be(buf: &mut Vec<u8>, value: u16) {
    buf.extend_from_slice(&value.to_be_bytes());
}

pub fn write_u16_le(buf: &mut Vec<u8>, value: u16) {
    buf.extend_from_slice(&value.to_le_bytes());
}

pub fn write_u32_be(buf: &mut Vec<u8>, value: u32) {
    buf.extend_from_slice(&value.to_be_bytes());
}

// ============================================
// Topic 3: Message Header
// Learn: Structured binary protocol with magic, version, length
// ============================================

#[derive(Debug, Clone, PartialEq)]
pub struct Header {
    pub magic: u16,
    pub version: u8,
    pub msg_type: u8,
    pub payload_len: u32,
}

impl Header {
    pub const SIZE: usize = 8;
    pub const MAGIC: u16 = 0xCAFE;

    pub fn new(version: u8, msg_type: u8, payload_len: u32) -> Self {
        Header { magic: Self::MAGIC, version, msg_type, payload_len }
    }

    pub fn encode(&self) -> Vec<u8> {
        let mut buf = Vec::with_capacity(Self::SIZE);
        write_u16_be(&mut buf, self.magic);
        write_u8(&mut buf, self.version);
        write_u8(&mut buf, self.msg_type);
        write_u32_be(&mut buf, self.payload_len);
        buf
    }

    pub fn decode(data: &[u8]) -> Result<Self, String> {
        if data.len() < Self::SIZE {
            return Err("too short".into());
        }
        let magic = read_u16_be(data, 0).unwrap();
        if magic != Self::MAGIC {
            return Err(format!("bad magic: 0x{:04X}", magic));
        }
        Ok(Header {
            magic,
            version: data[2],
            msg_type: data[3],
            payload_len: read_u32_be(data, 4).unwrap(),
        })
    }

    pub fn is_valid(&self) -> bool {
        self.magic == Self::MAGIC
    }
}

// ============================================
// Topic 4: TLV Encoding
// Learn: Tag-Length-Value, flexible binary format
// ============================================

#[derive(Debug, Clone, PartialEq)]
pub struct Tlv {
    pub tag: u8,
    pub value: Vec<u8>,
}

impl Tlv {
    pub fn new(tag: u8, value: Vec<u8>) -> Self {
        Tlv { tag, value }
    }

    pub fn encode(&self) -> Vec<u8> {
        let mut buf = Vec::new();
        buf.push(self.tag);
        buf.push(self.value.len() as u8);
        buf.extend_from_slice(&self.value);
        buf
    }

    pub fn decode(data: &[u8]) -> Result<(Tlv, usize), String> {
        if data.len() < 2 {
            return Err("too short for TLV".into());
        }
        let tag = data[0];
        let len = data[1] as usize;
        if data.len() < 2 + len {
            return Err("truncated value".into());
        }
        Ok((Tlv { tag, value: data[2..2+len].to_vec() }, 2 + len))
    }
}

pub fn encode_tlv_list(items: &[Tlv]) -> Vec<u8> {
    let mut buf = Vec::new();
    for item in items {
        buf.extend(item.encode());
    }
    buf
}

pub fn decode_tlv_list(data: &[u8]) -> Result<Vec<Tlv>, String> {
    let mut items = Vec::new();
    let mut offset = 0;
    while offset < data.len() {
        let (tlv, consumed) = Tlv::decode(&data[offset..])?;
        items.push(tlv);
        offset += consumed;
    }
    Ok(items)
}

// ============================================
// Topic 5: Varint Encoding
// Learn: Variable-length integer encoding (protobuf-style)
// ============================================

pub fn encode_varint(mut value: u64) -> Vec<u8> {
    let mut buf = Vec::new();
    loop {
        let mut byte = (value & 0x7F) as u8;
        value >>= 7;
        if value != 0 {
            byte |= 0x80;
        }
        buf.push(byte);
        if value == 0 { break; }
    }
    buf
}

pub fn decode_varint(data: &[u8]) -> Result<(u64, usize), String> {
    let mut result: u64 = 0;
    let mut shift = 0;
    for (i, &byte) in data.iter().enumerate() {
        result |= ((byte & 0x7F) as u64) << shift;
        if byte & 0x80 == 0 {
            return Ok((result, i + 1));
        }
        shift += 7;
        if shift >= 64 {
            return Err("varint too long".into());
        }
    }
    Err("unexpected end".into())
}

/// ZigZag encode a signed integer for efficient varint encoding.
pub fn zigzag_encode(n: i64) -> u64 {
    ((n << 1) ^ (n >> 63)) as u64
}

/// ZigZag decode back to signed.
pub fn zigzag_decode(n: u64) -> i64 {
    ((n >> 1) as i64) ^ -((n & 1) as i64)
}

// ============================================
// Topic 6: Checksum
// Learn: Data integrity verification
// ============================================

/// Simple XOR checksum.
pub fn xor_checksum(data: &[u8]) -> u8 {
    data.iter().fold(0u8, |acc, &b| acc ^ b)
}

/// Internet checksum (RFC 1071 simplified).
pub fn internet_checksum(data: &[u8]) -> u16 {
    let mut sum: u32 = 0;
    let mut i = 0;
    while i + 1 < data.len() {
        sum += u16::from_be_bytes([data[i], data[i+1]]) as u32;
        i += 2;
    }
    if i < data.len() {
        sum += (data[i] as u32) << 8;
    }
    while sum >> 16 != 0 {
        sum = (sum & 0xFFFF) + (sum >> 16);
    }
    !(sum as u16)
}

/// Verify data with its appended checksum.
pub fn verify_xor_checksum(data_with_checksum: &[u8]) -> bool {
    if data_with_checksum.is_empty() { return false; }
    xor_checksum(data_with_checksum) == 0
}
