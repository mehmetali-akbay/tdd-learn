// ============================================
// Level 10, Project 2: Bitwise Operations
// Learn: Bit manipulation, flags, packed data, binary arithmetic
// ============================================

// ============================================
// Topic 1: Bit Basics
// Learn: Set, clear, toggle, check individual bits
// ============================================

pub fn set_bit(value: u32, bit: u8) -> u32 {
    value | (1 << bit)
}

pub fn clear_bit(value: u32, bit: u8) -> u32 {
    value & !(1 << bit)
}

pub fn toggle_bit(value: u32, bit: u8) -> u32 {
    value ^ (1 << bit)
}

pub fn check_bit(value: u32, bit: u8) -> bool {
    (value >> bit) & 1 == 1
}

pub fn count_ones(value: u32) -> u32 {
    value.count_ones()
}

pub fn count_zeros(value: u32) -> u32 {
    value.count_zeros()
}

// ============================================
// Topic 2: Bit Flags
// Learn: Using bits as boolean flags, set operations
// ============================================

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct BitFlags(u32);

impl BitFlags {
    pub fn new() -> Self {
        BitFlags(0)
    }

    pub fn from_raw(bits: u32) -> Self {
        BitFlags(bits)
    }

    pub fn raw(&self) -> u32 {
        self.0
    }

    pub fn set(&mut self, flag: u32) {
        self.0 |= flag;
    }

    pub fn unset(&mut self, flag: u32) {
        self.0 &= !flag;
    }

    pub fn has(&self, flag: u32) -> bool {
        self.0 & flag == flag
    }

    pub fn toggle(&mut self, flag: u32) {
        self.0 ^= flag;
    }

    pub fn is_empty(&self) -> bool {
        self.0 == 0
    }

    pub fn union(&self, other: &BitFlags) -> BitFlags {
        BitFlags(self.0 | other.0)
    }

    pub fn intersection(&self, other: &BitFlags) -> BitFlags {
        BitFlags(self.0 & other.0)
    }

    pub fn difference(&self, other: &BitFlags) -> BitFlags {
        BitFlags(self.0 & !other.0)
    }
}

impl Default for BitFlags {
    fn default() -> Self { Self::new() }
}

// ============================================
// Topic 3: Bit Math
// Learn: Power of two checks, log2, bit counting
// ============================================

pub fn is_power_of_two(n: u32) -> bool {
    n > 0 && (n & (n - 1)) == 0
}

pub fn next_power_of_two(n: u32) -> u32 {
    if n == 0 { return 1; }
    n.next_power_of_two()
}

pub fn floor_log2(n: u32) -> u32 {
    assert!(n > 0);
    31 - n.leading_zeros()
}

pub fn leading_zeros(n: u32) -> u32 {
    n.leading_zeros()
}

pub fn trailing_zeros(n: u32) -> u32 {
    n.trailing_zeros()
}

/// Extract bits [low..=high] from value.
pub fn extract_bits(value: u32, low: u8, high: u8) -> u32 {
    let mask = ((1u64 << (high - low + 1)) - 1) as u32;
    (value >> low) & mask
}

// ============================================
// Topic 4: Packed Structs
// Learn: Packing multiple fields into a single integer
// ============================================

/// Pack an RGB color (8 bits each) into a u32.
pub fn pack_rgb(r: u8, g: u8, b: u8) -> u32 {
    ((r as u32) << 16) | ((g as u32) << 8) | (b as u32)
}

/// Unpack a u32 into (r, g, b).
pub fn unpack_rgb(packed: u32) -> (u8, u8, u8) {
    let r = ((packed >> 16) & 0xFF) as u8;
    let g = ((packed >> 8) & 0xFF) as u8;
    let b = (packed & 0xFF) as u8;
    (r, g, b)
}

/// Pack two u16 values into a u32.
pub fn pack_u16_pair(high: u16, low: u16) -> u32 {
    ((high as u32) << 16) | (low as u32)
}

/// Unpack a u32 into two u16 values.
pub fn unpack_u16_pair(packed: u32) -> (u16, u16) {
    let high = (packed >> 16) as u16;
    let low = (packed & 0xFFFF) as u16;
    (high, low)
}

/// Pack four u8 values into a u32.
pub fn pack_u8_quad(a: u8, b: u8, c: u8, d: u8) -> u32 {
    ((a as u32) << 24) | ((b as u32) << 16) | ((c as u32) << 8) | (d as u32)
}

/// Unpack a u32 into four u8 values.
pub fn unpack_u8_quad(packed: u32) -> (u8, u8, u8, u8) {
    (
        (packed >> 24) as u8,
        ((packed >> 16) & 0xFF) as u8,
        ((packed >> 8) & 0xFF) as u8,
        (packed & 0xFF) as u8,
    )
}

// ============================================
// Topic 5: Bitwise Algorithms
// Learn: Reverse, rotate, parity, Gray code
// ============================================

pub fn reverse_bits(mut n: u32) -> u32 {
    let mut result = 0u32;
    for _ in 0..32 {
        result = (result << 1) | (n & 1);
        n >>= 1;
    }
    result
}

pub fn rotate_left(value: u32, amount: u32) -> u32 {
    value.rotate_left(amount)
}

pub fn rotate_right(value: u32, amount: u32) -> u32 {
    value.rotate_right(amount)
}

/// Check if the number of set bits is even.
pub fn has_even_parity(n: u32) -> bool {
    n.count_ones() % 2 == 0
}

/// Convert binary to Gray code.
pub fn to_gray_code(n: u32) -> u32 {
    n ^ (n >> 1)
}

/// Convert Gray code back to binary.
pub fn from_gray_code(mut gray: u32) -> u32 {
    let mut n = gray;
    gray >>= 1;
    while gray != 0 {
        n ^= gray;
        gray >>= 1;
    }
    n
}

// ============================================
// Topic 6: Advanced — BitSet
// Learn: Arbitrary-size bit set using Vec<u64>
// ============================================

pub struct BitSet {
    blocks: Vec<u64>,
}

impl BitSet {
    pub fn new(size: usize) -> Self {
        let block_count = (size + 63) / 64;
        BitSet { blocks: vec![0; block_count] }
    }

    pub fn set(&mut self, bit: usize) {
        let block = bit / 64;
        let offset = bit % 64;
        if block < self.blocks.len() {
            self.blocks[block] |= 1 << offset;
        }
    }

    pub fn clear(&mut self, bit: usize) {
        let block = bit / 64;
        let offset = bit % 64;
        if block < self.blocks.len() {
            self.blocks[block] &= !(1 << offset);
        }
    }

    pub fn test(&self, bit: usize) -> bool {
        let block = bit / 64;
        let offset = bit % 64;
        if block < self.blocks.len() {
            (self.blocks[block] >> offset) & 1 == 1
        } else {
            false
        }
    }

    pub fn count(&self) -> usize {
        self.blocks.iter().map(|b| b.count_ones() as usize).sum()
    }

    pub fn is_empty(&self) -> bool {
        self.blocks.iter().all(|&b| b == 0)
    }

    pub fn union(&self, other: &BitSet) -> BitSet {
        let len = self.blocks.len().max(other.blocks.len());
        let mut blocks = vec![0u64; len];
        for i in 0..len {
            let a = self.blocks.get(i).copied().unwrap_or(0);
            let b = other.blocks.get(i).copied().unwrap_or(0);
            blocks[i] = a | b;
        }
        BitSet { blocks }
    }

    pub fn intersection(&self, other: &BitSet) -> BitSet {
        let len = self.blocks.len().min(other.blocks.len());
        let mut blocks = vec![0u64; len];
        for i in 0..len {
            blocks[i] = self.blocks[i] & other.blocks[i];
        }
        BitSet { blocks }
    }
}
