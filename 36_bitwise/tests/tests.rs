use bitwise::*;

// ===== Topic 1: Bit Basics =====

#[test]
fn test_set_bit() {
    assert_eq!(set_bit(0, 0), 1);
    assert_eq!(set_bit(0, 3), 8);
    assert_eq!(set_bit(0b1010, 0), 0b1011);
}

#[test]
fn test_clear_bit() {
    assert_eq!(clear_bit(0b1111, 1), 0b1101);
}

#[test]
fn test_toggle_bit() {
    assert_eq!(toggle_bit(0b1010, 1), 0b1000);
    assert_eq!(toggle_bit(0b1010, 0), 0b1011);
}

#[test]
fn test_check_bit() {
    assert!(check_bit(0b1010, 1));
    assert!(!check_bit(0b1010, 0));
}

#[test]
fn test_count_ones_zeros() {
    assert_eq!(count_ones(0b1010), 2);
    assert_eq!(count_zeros(0b1010), 30);
}

// ===== Topic 2: Bit Flags =====

#[test]
fn test_bitflags_basic() {
    let mut flags = BitFlags::new();
    assert!(flags.is_empty());
    flags.set(0b0001);
    flags.set(0b0100);
    assert!(flags.has(0b0001));
    assert!(flags.has(0b0100));
    assert!(!flags.has(0b0010));
}

#[test]
fn test_bitflags_toggle() {
    let mut flags = BitFlags::from_raw(0b1010);
    flags.toggle(0b1111);
    assert_eq!(flags.raw(), 0b0101);
}

#[test]
fn test_bitflags_set_ops() {
    let a = BitFlags::from_raw(0b1100);
    let b = BitFlags::from_raw(0b1010);
    assert_eq!(a.union(&b).raw(), 0b1110);
    assert_eq!(a.intersection(&b).raw(), 0b1000);
    assert_eq!(a.difference(&b).raw(), 0b0100);
}

// ===== Topic 3: Bit Math =====

#[test]
fn test_power_of_two() {
    assert!(is_power_of_two(8));
    assert!(!is_power_of_two(6));
    assert!(!is_power_of_two(0));
}

#[test]
fn test_next_power_of_two() {
    assert_eq!(next_power_of_two(5), 8);
    assert_eq!(next_power_of_two(8), 8);
}

#[test]
fn test_floor_log2() {
    assert_eq!(floor_log2(8), 3);
    assert_eq!(floor_log2(15), 3);
    assert_eq!(floor_log2(16), 4);
}

#[test]
fn test_extract_bits() {
    assert_eq!(extract_bits(0b11010110, 2, 5), 0b0101);
}

// ===== Topic 4: Packed Structs =====

#[test]
fn test_pack_unpack_rgb() {
    let packed = pack_rgb(255, 128, 0);
    assert_eq!(unpack_rgb(packed), (255, 128, 0));
}

#[test]
fn test_pack_u16_pair() {
    let packed = pack_u16_pair(0x1234, 0x5678);
    assert_eq!(unpack_u16_pair(packed), (0x1234, 0x5678));
}

#[test]
fn test_pack_u8_quad() {
    let packed = pack_u8_quad(1, 2, 3, 4);
    assert_eq!(unpack_u8_quad(packed), (1, 2, 3, 4));
}

// ===== Topic 5: Algorithms =====

#[test]
fn test_reverse_bits() {
    assert_eq!(reverse_bits(1), 1 << 31);
}

#[test]
fn test_rotate() {
    let v: u32 = 0x80000001;
    assert_eq!(rotate_left(v, 1), 0x00000003);
}

#[test]
fn test_parity() {
    assert!(has_even_parity(0b1010));
    assert!(!has_even_parity(0b1011));
}

#[test]
fn test_gray_code_roundtrip() {
    for i in 0..100 {
        assert_eq!(from_gray_code(to_gray_code(i)), i);
    }
}

// ===== Topic 6: BitSet =====

#[test]
fn test_bitset_basic() {
    let mut bs = BitSet::new(128);
    bs.set(0);
    bs.set(64);
    bs.set(127);
    assert!(bs.test(0));
    assert!(bs.test(64));
    assert!(bs.test(127));
    assert!(!bs.test(1));
    assert_eq!(bs.count(), 3);
}

#[test]
fn test_bitset_clear() {
    let mut bs = BitSet::new(64);
    bs.set(10);
    bs.clear(10);
    assert!(!bs.test(10));
}

#[test]
fn test_bitset_union_intersection() {
    let mut a = BitSet::new(64);
    let mut b = BitSet::new(64);
    a.set(1);
    a.set(2);
    b.set(2);
    b.set(3);
    assert_eq!(a.union(&b).count(), 3);
    assert_eq!(a.intersection(&b).count(), 1);
}

#[test]
fn test_bitset_empty() {
    let bs = BitSet::new(64);
    assert!(bs.is_empty());
    assert_eq!(bs.count(), 0);
}
