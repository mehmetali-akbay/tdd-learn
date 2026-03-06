use ownership::*;

// ===== Topic 1: Move Semantics =====

#[test]
fn test_take_and_measure() {
    let s = String::from("hello");
    assert_eq!(take_and_measure(s), 5);
    // s is no longer usable here — it was moved!
}

#[test]
fn test_take_and_shout() {
    let s = String::from("hello");
    assert_eq!(take_and_shout(s), "HELLO");
}

#[test]
fn test_create_greeting() {
    let g = create_greeting();
    assert_eq!(g, "Hello from Rust!");
}

#[test]
fn test_push_and_return() {
    let v = vec![1, 2, 3];
    let result = push_and_return(v, 4);
    assert_eq!(result, vec![1, 2, 3, 4]);
}

#[test]
fn test_swap_values() {
    let a = String::from("first");
    let b = String::from("second");
    let (new_a, new_b) = swap_values(a, b);
    assert_eq!(new_a, "second");
    assert_eq!(new_b, "first");
}

// ===== Topic 2: Clone and Copy =====

#[test]
fn test_clone_string() {
    let original = String::from("hello");
    let cloned = clone_string(&original);
    assert_eq!(cloned, "hello");
    // original is still usable because we borrowed it
    assert_eq!(original, "hello");
}

#[test]
fn test_add_copies() {
    let a = 5;
    let b = 3;
    let sum = add_copies(a, b);
    assert_eq!(sum, 8);
    // a and b are still usable because i32 implements Copy
    assert_eq!(a, 5);
    assert_eq!(b, 3);
}

#[test]
fn test_clone_and_reverse() {
    let v = vec![1, 2, 3];
    let (original, reversed) = clone_and_reverse(&v);
    assert_eq!(original, vec![1, 2, 3]);
    assert_eq!(reversed, vec![3, 2, 1]);
    // v is still usable because we only borrowed it
    assert_eq!(v, vec![1, 2, 3]);
}

#[test]
fn test_make_labeled_copy() {
    assert_eq!(make_labeled_copy("original"), "original (copy)");
    assert_eq!(make_labeled_copy("data"), "data (copy)");
}

#[test]
fn test_double_string() {
    assert_eq!(double_string("abc"), "abcabc");
    assert_eq!(double_string(""), "");
    assert_eq!(double_string("x"), "xx");
}

// ===== Topic 3: Immutable Borrowing =====

#[test]
fn test_borrow_and_count() {
    let s = String::from("hello");
    let len = borrow_and_count(&s);
    assert_eq!(len, 5);
    // s is still usable — we only borrowed it
    assert_eq!(s, "hello");
}

#[test]
fn test_borrow_and_sum() {
    let v = vec![1, 2, 3, 4, 5];
    let total = borrow_and_sum(&v);
    assert_eq!(total, 15);
    assert_eq!(v, vec![1, 2, 3, 4, 5]); // v unchanged
}

#[test]
fn test_contains_rust() {
    assert!(contains_rust("I love Rust!"));
    assert!(contains_rust("RUST is great"));
    assert!(!contains_rust("Python"));
}

#[test]
fn test_longer_string() {
    assert_eq!(longer_string("hello", "hi"), "hello");
    assert_eq!(longer_string("ab", "abcd"), "abcd");
    assert_eq!(longer_string("same", "size"), "same"); // equal = first
}

#[test]
fn test_filter_positive() {
    let v = vec![-2, -1, 0, 1, 2, 3];
    assert_eq!(filter_positive(&v), vec![1, 2, 3]);
    assert_eq!(filter_positive(&[-1, -2]), vec![]);
    assert_eq!(v, vec![-2, -1, 0, 1, 2, 3]); // v unchanged
}

// ===== Topic 4: Mutable Borrowing =====

#[test]
fn test_push_value() {
    let mut v = vec![1, 2, 3];
    push_value(&mut v, 4);
    assert_eq!(v, vec![1, 2, 3, 4]);
}

#[test]
fn test_make_uppercase() {
    let mut s = String::from("hello");
    make_uppercase(&mut s);
    assert_eq!(s, "HELLO");
}

#[test]
fn test_increment_all() {
    let mut v = vec![1, 2, 3];
    increment_all(&mut v);
    assert_eq!(v, vec![2, 3, 4]);
}

#[test]
fn test_remove_negatives() {
    let mut v = vec![-2, -1, 0, 1, 2];
    remove_negatives(&mut v);
    assert_eq!(v, vec![0, 1, 2]);
}

#[test]
fn test_append_world() {
    let mut s = String::from("hello");
    append_world(&mut s);
    assert_eq!(s, "hello world");
}

// ===== Topic 5: Ownership in Functions =====

#[test]
fn test_process_and_keep() {
    let s = String::from("hello");
    let (original, reversed) = process_and_keep(s);
    assert_eq!(original, "hello");
    assert_eq!(reversed, "olleh");
}

#[test]
fn test_transform_each() {
    fn double(x: i32) -> i32 {
        x * 2
    }
    assert_eq!(transform_each(&[1, 2, 3], double), vec![2, 4, 6]);
    assert_eq!(transform_each(&[1, 2, 3], |x| x + 10), vec![11, 12, 13]);
}

#[test]
fn test_longest() {
    assert_eq!(longest("long string", "short"), "long string");
    assert_eq!(longest("ab", "abc"), "abc");
}

#[test]
fn test_sort_owned() {
    assert_eq!(sort_owned(vec![3, 1, 4, 1, 5]), vec![1, 1, 3, 4, 5]);
    assert_eq!(sort_owned(vec![]), vec![]);
}

#[test]
fn test_join_with_dash() {
    assert_eq!(join_with_dash("hello", "world"), "hello-world");
    assert_eq!(join_with_dash("foo", "bar"), "foo-bar");
}

// ===== Topic 6: Advanced Ownership Challenges =====

#[test]
fn test_stack_basic() {
    let mut stack = Stack::new();
    assert!(stack.is_empty());
    assert_eq!(stack.size(), 0);

    stack.push(10);
    stack.push(20);
    stack.push(30);
    assert_eq!(stack.size(), 3);
    assert!(!stack.is_empty());
}

#[test]
fn test_stack_push_pop() {
    let mut stack = Stack::new();
    stack.push(1);
    stack.push(2);
    stack.push(3);
    assert_eq!(stack.pop(), Some(3));
    assert_eq!(stack.pop(), Some(2));
    assert_eq!(stack.pop(), Some(1));
    assert_eq!(stack.pop(), None);
}

#[test]
fn test_stack_peek() {
    let mut stack = Stack::new();
    assert_eq!(stack.peek(), None);
    stack.push(42);
    assert_eq!(stack.peek(), Some(&42));
    // peek doesn't remove
    assert_eq!(stack.size(), 1);
}

#[test]
fn test_string_builder() {
    let mut sb = StringBuilder::new();
    assert!(sb.is_empty());
    sb.append("hello");
    sb.append(" world");
    assert_eq!(sb.len(), 11);
    assert_eq!(sb.build(), "hello world");
}

#[test]
fn test_string_builder_chaining() {
    let mut sb = StringBuilder::new();
    sb.append("hello").append(" ").append("world");
    assert_eq!(sb.build(), "hello world");
}

#[test]
fn test_string_builder_lines() {
    let mut sb = StringBuilder::new();
    sb.append_line("line 1").append_line("line 2");
    assert_eq!(sb.build(), "line 1\nline 2\n");
}

#[test]
fn test_filter_and_join() {
    let words = vec![
        "rust".to_string(),
        "ruby".to_string(),
        "python".to_string(),
        "raku".to_string(),
    ];
    assert_eq!(filter_and_join(words, "ru"), "rust, ruby");
}

#[test]
fn test_slugify() {
    assert_eq!(slugify("  Hello World  ".to_string()), "hello-world");
    assert_eq!(slugify("Rust Is Great".to_string()), "rust-is-great");
}

#[test]
fn test_numbered_list() {
    let items = vec![
        "apple".to_string(),
        "banana".to_string(),
        "cherry".to_string(),
    ];
    assert_eq!(numbered_list(items), "1. apple\n2. banana\n3. cherry");
}

#[test]
fn test_filter_refs() {
    let words = vec![
        "hi".to_string(),
        "hello".to_string(),
        "hey".to_string(),
        "howdy".to_string(),
    ];
    let long_words = filter_refs(&words, 4);
    assert_eq!(long_words, vec!["hello", "howdy"]);
    // original vec is still usable
    assert_eq!(words.len(), 4);
}

#[test]
fn test_merge_borrowed() {
    let a = [5, 1, 3];
    let b = [4, 2, 6];
    assert_eq!(merge_borrowed(&a, &b), vec![1, 2, 3, 4, 5, 6]);
    // a and b still usable
    assert_eq!(a, [5, 1, 3]);
}

#[test]
fn test_owned_split() {
    let result = owned_split("a:b:c", ':');
    assert_eq!(result, vec!["a", "b", "c"]);
    assert_eq!(owned_split("single", ':'), vec!["single"]);
}

// ===== Topic 7: Extra Practice =====

#[test]
fn test_ownership_interleave() {
    assert_eq!(ownership_interleave(vec![1, 2], vec![10, 20, 30]), vec![1, 10, 2, 20, 30]);
}

#[test]
fn test_clone_filtered() {
    let data = vec!["hello".into(), "hi".into(), "hey".into()];
    let result = clone_filtered(&data, |s| s.len() > 2);
    assert_eq!(result, vec!["hello", "hey"]);
}

#[test]
fn test_extract_first() {
    let items = vec!["a".into(), "b".into(), "c".into()];
    let (first, rest) = extract_first(items);
    assert_eq!(first, Some("a".into()));
    assert_eq!(rest, vec!["b", "c"]);
}

#[test]
fn test_swap_pair() {
    let (mut a, mut b) = (10, 20);
    swap_pair(&mut a, &mut b);
    assert_eq!(a, 20);
    assert_eq!(b, 10);
}

#[test]
fn test_partition_owned() {
    let items = vec!["apple".into(), "hi".into(), "banana".into()];
    let (long, short) = partition_owned(items, |s| s.len() > 3);
    assert_eq!(long, vec!["apple", "banana"]);
    assert_eq!(short, vec!["hi"]);
}
