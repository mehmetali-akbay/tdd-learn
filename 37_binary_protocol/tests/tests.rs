use binary_protocol::*;

// ===== Topic 1: Byte Reading =====

#[test]
fn test_read_u8() {
    assert_eq!(read_u8(&[0x42, 0xFF], 0), Some(0x42));
    assert_eq!(read_u8(&[0x42], 1), None);
}

#[test]
fn test_read_u16_be_le() {
    let data = [0x01, 0x02];
    assert_eq!(read_u16_be(&data, 0), Some(0x0102));
    assert_eq!(read_u16_le(&data, 0), Some(0x0201));
}

#[test]
fn test_read_u32_be() {
    let data = [0x00, 0x00, 0x01, 0x00];
    assert_eq!(read_u32_be(&data, 0), Some(256));
}

#[test]
fn test_read_out_of_bounds() {
    assert_eq!(read_u16_be(&[0x01], 0), None);
    assert_eq!(read_u32_be(&[0x01, 0x02, 0x03], 0), None);
}

// ===== Topic 2: Byte Writing =====

#[test]
fn test_write_u16_be() {
    let mut buf = Vec::new();
    write_u16_be(&mut buf, 0x1234);
    assert_eq!(buf, vec![0x12, 0x34]);
}

#[test]
fn test_write_u32_be() {
    let mut buf = Vec::new();
    write_u32_be(&mut buf, 0xDEADBEEF);
    assert_eq!(buf, vec![0xDE, 0xAD, 0xBE, 0xEF]);
}

// ===== Topic 3: Message Header =====

#[test]
fn test_header_encode_decode() {
    let h = Header::new(1, 2, 100);
    let encoded = h.encode();
    assert_eq!(encoded.len(), Header::SIZE);
    let decoded = Header::decode(&encoded).unwrap();
    assert_eq!(decoded, h);
}

#[test]
fn test_header_bad_magic() {
    let data = [0x00, 0x00, 0x01, 0x02, 0x00, 0x00, 0x00, 0x64];
    assert!(Header::decode(&data).is_err());
}

#[test]
fn test_header_too_short() {
    assert!(Header::decode(&[0xCA, 0xFE]).is_err());
}

// ===== Topic 4: TLV Encoding =====

#[test]
fn test_tlv_encode_decode() {
    let tlv = Tlv::new(1, vec![0x41, 0x42, 0x43]);
    let encoded = tlv.encode();
    let (decoded, consumed) = Tlv::decode(&encoded).unwrap();
    assert_eq!(decoded, tlv);
    assert_eq!(consumed, 5);
}

#[test]
fn test_tlv_list() {
    let items = vec![
        Tlv::new(1, vec![0x01]),
        Tlv::new(2, vec![0x02, 0x03]),
    ];
    let encoded = encode_tlv_list(&items);
    let decoded = decode_tlv_list(&encoded).unwrap();
    assert_eq!(decoded, items);
}

#[test]
fn test_tlv_empty_value() {
    let tlv = Tlv::new(5, vec![]);
    let encoded = tlv.encode();
    let (decoded, _) = Tlv::decode(&encoded).unwrap();
    assert_eq!(decoded.value, vec![]);
}

// ===== Topic 5: Varint Encoding =====

#[test]
fn test_varint_small() {
    let encoded = encode_varint(42);
    assert_eq!(encoded, vec![42]);
    let (decoded, consumed) = decode_varint(&encoded).unwrap();
    assert_eq!(decoded, 42);
    assert_eq!(consumed, 1);
}

#[test]
fn test_varint_large() {
    let encoded = encode_varint(300);
    assert!(encoded.len() > 1);
    let (decoded, _) = decode_varint(&encoded).unwrap();
    assert_eq!(decoded, 300);
}

#[test]
fn test_zigzag() {
    assert_eq!(zigzag_encode(0), 0);
    assert_eq!(zigzag_encode(-1), 1);
    assert_eq!(zigzag_encode(1), 2);
    for n in [-100, -1, 0, 1, 100] {
        assert_eq!(zigzag_decode(zigzag_encode(n)), n);
    }
}

// ===== Topic 6: Checksum =====

#[test]
fn test_xor_checksum() {
    assert_eq!(xor_checksum(&[0x01, 0x02, 0x03]), 0x01 ^ 0x02 ^ 0x03);
}

#[test]
fn test_verify_xor_checksum() {
    let data = vec![0x01, 0x02, 0x03];
    let cs = xor_checksum(&data);
    let mut with_cs = data;
    with_cs.push(cs);
    assert!(verify_xor_checksum(&with_cs));
}

#[test]
fn test_internet_checksum() {
    let data = vec![0x00, 0x01, 0x00, 0x02];
    let cs = internet_checksum(&data);
    assert_ne!(cs, 0);
}

// ===== Edge Cases =====

#[test]
fn test_varint_zero() {
    let encoded = encode_varint(0);
    assert_eq!(encoded, vec![0]);
    let (decoded, _) = decode_varint(&encoded).unwrap();
    assert_eq!(decoded, 0);
}

#[test]
fn test_header_validity() {
    let h = Header::new(1, 0, 0);
    assert!(h.is_valid());
}
