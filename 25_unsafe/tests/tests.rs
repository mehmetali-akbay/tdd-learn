use unsafe_rust::*;

#[test]
fn test_read_via_raw_pointer() {
    assert_eq!(read_via_raw_pointer(&42), 42);
}
#[test]
fn test_modify_via_raw_pointer() {
    let mut v = 10;
    modify_via_raw_pointer(&mut v, 20);
    assert_eq!(v, 20);
}
#[test]
fn test_address_of() {
    assert!(address_of(&42) > 0);
}
#[test]
fn test_same_address_true() {
    let v = 42;
    assert!(same_address(&v, &v));
}
#[test]
fn test_same_address_false() {
    let a = 42;
    let b = 42;
    assert!(!same_address(&a, &b));
}
#[test]
fn test_read_ptr() {
    let v = 99;
    assert_eq!(unsafe { read_ptr(&v as *const i32) }, 99);
}
#[test]
fn test_write_ptr() {
    let mut v = 10;
    unsafe { write_ptr(&mut v as *mut i32, 20) };
    assert_eq!(v, 20);
}
#[test]
fn test_safe_swap() {
    let mut a = 1;
    let mut b = 2;
    safe_swap(&mut a, &mut b);
    assert_eq!(a, 2);
    assert_eq!(b, 1);
}
#[test]
fn test_find_index() {
    assert_eq!(find_index(&[10, 20, 30, 40], 30), Some(2));
    assert_eq!(find_index(&[10, 20, 30], 99), None);
}
#[test]
fn test_sum_with_pointers() {
    assert_eq!(sum_with_pointers(&[1, 2, 3, 4, 5]), 15);
    assert_eq!(sum_with_pointers(&[]), 0);
}
#[test]
fn test_copy_elements() {
    let src = [1, 2, 3, 4, 5];
    let mut dst = [0; 3];
    assert_eq!(copy_elements(&src, &mut dst), 3);
    assert_eq!(dst, [1, 2, 3]);
}
#[test]
fn test_counter() {
    reset_counter();
    assert_eq!(get_counter(), 0);
    assert_eq!(increment_counter(), 1);
    assert_eq!(increment_counter(), 2);
    assert_eq!(get_counter(), 2);
    reset_counter();
}
#[test]
fn test_max_retries() {
    set_max_retries(5);
    assert_eq!(get_max_retries(), 5);
    set_max_retries(3);
}
#[test]
fn test_sendable_data() {
    assert_eq!(SendableData::new("hello").get(), "hello");
}
#[test]
fn test_check_send_sync() {
    assert!(check_send_sync());
}
#[test]
fn test_not_sync_wrapper() {
    assert_eq!(NotSyncWrapper::new(42).get(), 42);
}
#[test]
fn test_split_at_mut() {
    let mut v = vec![1, 2, 3, 4, 5];
    let (l, r) = my_split_at_mut(&mut v, 3);
    assert_eq!(l, &[1, 2, 3]);
    assert_eq!(r, &[4, 5]);
}
#[test]
fn test_split_at_mut_start() {
    let mut v = vec![1, 2, 3];
    let (l, r) = my_split_at_mut(&mut v, 0);
    assert!(l.is_empty());
    assert_eq!(r, &[1, 2, 3]);
}
#[test]
fn test_split_at_mut_end() {
    let mut v = vec![1, 2, 3];
    let (l, r) = my_split_at_mut(&mut v, 3);
    assert_eq!(l, &[1, 2, 3]);
    assert!(r.is_empty());
}
#[test]
fn test_arena() {
    let mut a = Arena::new();
    let i0 = a.alloc(10);
    let i1 = a.alloc(20);
    assert_eq!(a.get(i0), Some(&10));
    assert_eq!(a.get(i1), Some(&20));
    assert_eq!(a.len(), 2);
}
#[test]
fn test_arena_all() {
    let mut a = Arena::new();
    a.alloc(1);
    a.alloc(2);
    a.alloc(3);
    assert_eq!(a.all(), &[1, 2, 3]);
}
