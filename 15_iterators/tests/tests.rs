use iterators::*;

// ===== Topic 1: Iterator Basics =====

#[test]
fn test_sum_of_squares() {
    assert_eq!(sum_of_squares(3), 14); // 1 + 4 + 9
    assert_eq!(sum_of_squares(0), 0);
    assert_eq!(sum_of_squares(1), 1);
}

#[test]
fn test_product() {
    assert_eq!(product(&[1, 2, 3, 4]), 24);
    assert_eq!(product(&[]), 1);
    assert_eq!(product(&[5]), 5);
}

#[test]
fn test_flatten() {
    assert_eq!(
        flatten(&[vec![1, 2], vec![3], vec![4, 5, 6]]),
        vec![1, 2, 3, 4, 5, 6]
    );
    assert_eq!(flatten::<i32>(&[]), Vec::<i32>::new());
}

#[test]
fn test_zip_sum() {
    assert_eq!(zip_sum(&[1, 2, 3], &[10, 20, 30]), vec![11, 22, 33]);
    assert_eq!(zip_sum(&[1, 2, 3], &[10, 20]), vec![11, 22]); // shorter
}

#[test]
fn test_even_indexed() {
    assert_eq!(even_indexed(&[10, 20, 30, 40, 50]), vec![10, 30, 50]);
    assert_eq!(even_indexed::<i32>(&[]), Vec::<i32>::new());
}

#[test]
fn test_running_max() {
    assert_eq!(
        running_max(&[3, 1, 4, 1, 5, 9, 2]),
        vec![3, 3, 4, 4, 5, 9, 9]
    );
    assert_eq!(running_max(&[]), Vec::<i32>::new());
}

// ===== Topic 2: Chained Transformations =====

#[test]
fn test_unique_words_sorted() {
    assert_eq!(
        unique_words_sorted("hello world Hello WORLD foo"),
        vec!["foo", "hello", "world"]
    );
}

#[test]
fn test_take_while_ascending() {
    assert_eq!(take_while_ascending(&[1, 3, 5, 2, 7]), vec![1, 3, 5]);
    assert_eq!(take_while_ascending(&[5, 4, 3]), vec![5]);
    assert_eq!(take_while_ascending(&[]), Vec::<i32>::new());
}

#[test]
fn test_skip_leading_zeros() {
    assert_eq!(skip_leading_zeros(&[0, 0, 0, 1, 2, 0, 3]), vec![1, 2, 0, 3]);
    assert_eq!(skip_leading_zeros(&[1, 2, 3]), vec![1, 2, 3]);
}

#[test]
fn test_group_consecutive() {
    assert_eq!(
        group_consecutive(&[1, 1, 2, 2, 2, 3]),
        vec![vec![1, 1], vec![2, 2, 2], vec![3]]
    );
    assert_eq!(group_consecutive(&[]), Vec::<Vec<i32>>::new());
}

// ===== Topic 3: Counter =====

#[test]
fn test_counter_basic() {
    let nums: Vec<i32> = Counter::new(0, 1).take(5).collect();
    assert_eq!(nums, vec![0, 1, 2, 3, 4]);
}

#[test]
fn test_counter_step() {
    let nums: Vec<i32> = Counter::new(0, 3).take(4).collect();
    assert_eq!(nums, vec![0, 3, 6, 9]);
}

#[test]
fn test_counter_with_limit() {
    let nums: Vec<i32> = Counter::with_limit(0, 2, 10).collect();
    assert_eq!(nums, vec![0, 2, 4, 6, 8]);
}

#[test]
fn test_counter_negative_step() {
    let nums: Vec<i32> = Counter::with_limit(10, -2, 0).collect();
    assert_eq!(nums, vec![10, 8, 6, 4, 2]);
}

#[test]
fn test_counter_with_iterator_methods() {
    let sum: i32 = Counter::with_limit(1, 1, 6).filter(|x| x % 2 == 0).sum();
    assert_eq!(sum, 6); // 2 + 4
}

// ===== Topic 4: Fibonacci =====

#[test]
fn test_first_n_fib() {
    assert_eq!(first_n_fib(8), vec![0, 1, 1, 2, 3, 5, 8, 13]);
    assert_eq!(first_n_fib(0), vec![]);
    assert_eq!(first_n_fib(1), vec![0]);
}

#[test]
fn test_fib_sum_below() {
    assert_eq!(fib_sum_below(10), 20); // 0+1+1+2+3+5+8 = 20
}

#[test]
fn test_fib_with_filter() {
    // Even Fibonacci numbers below 100
    let even_fibs: Vec<u64> = Fibonacci::new()
        .take_while(|&x| x < 100)
        .filter(|x| x % 2 == 0)
        .collect();
    assert_eq!(even_fibs, vec![0, 2, 8, 34]);
}

// ===== Topic 5: Chunks =====

#[test]
fn test_chunk_slice() {
    assert_eq!(
        chunk_slice(&[1, 2, 3, 4, 5], 2),
        vec![vec![1, 2], vec![3, 4], vec![5]]
    );
    assert_eq!(chunk_slice(&[1, 2, 3], 3), vec![vec![1, 2, 3]]);
    assert_eq!(chunk_slice::<i32>(&[], 2), Vec::<Vec<i32>>::new());
}

#[test]
fn test_chunk_slice_large_chunk() {
    assert_eq!(chunk_slice(&[1, 2], 10), vec![vec![1, 2]]);
}

#[test]
fn test_chunks_with_strings() {
    let words = vec![
        "a".to_string(),
        "b".to_string(),
        "c".to_string(),
        "d".to_string(),
    ];
    let chunks: Vec<Vec<String>> = Chunks::new(words, 2).collect();
    assert_eq!(chunks.len(), 2);
    assert_eq!(chunks[0], vec!["a", "b"]);
    assert_eq!(chunks[1], vec!["c", "d"]);
}

// ===== Topic 6: Advanced Combinators =====

#[test]
fn test_running_sum() {
    assert_eq!(running_sum(&[1, 2, 3, 4]), vec![1, 3, 6, 10]);
    assert_eq!(running_sum(&[]), Vec::<i32>::new());
}

#[test]
fn test_interleave() {
    assert_eq!(
        interleave(&[1, 2, 3], &[10, 20, 30]),
        vec![1, 10, 2, 20, 3, 30]
    );
    assert_eq!(interleave(&[1, 2, 3], &[10, 20]), vec![1, 10, 2, 20, 3]);
    assert_eq!(interleave(&[1], &[10, 20, 30]), vec![1, 10, 20, 30]);
}

#[test]
fn test_chain_all() {
    assert_eq!(
        chain_all(vec![vec![1, 2], vec![3], vec![4, 5]]),
        vec![1, 2, 3, 4, 5]
    );
}

#[test]
fn test_window_averages() {
    let avgs = window_averages(&[1.0, 2.0, 3.0, 4.0, 5.0], 3);
    assert_eq!(avgs.len(), 3);
    assert!((avgs[0] - 2.0).abs() < 0.001);
    assert!((avgs[1] - 3.0).abs() < 0.001);
    assert!((avgs[2] - 4.0).abs() < 0.001);
    assert_eq!(window_averages(&[1.0], 3), Vec::<f64>::new());
}

#[test]
fn test_pairwise_diff() {
    assert_eq!(pairwise_diff(&[1, 3, 6, 10]), vec![2, 3, 4]);
    assert_eq!(pairwise_diff(&[5]), Vec::<i32>::new());
}


// ===== Topic 7: Extra Practice =====

#[test]
fn test_unique_from_both() {
    let result = unique_from_both(&[1, 2, 3], &[2, 3, 4]);
    assert_eq!(result, vec![1, 2, 3, 4]);
}

#[test]
fn test_cumulative_max() {
    assert_eq!(cumulative_max(&[1, 3, 2, 5, 4]), vec![1, 3, 3, 5, 5]);
}

#[test]
fn test_group_runs() {
    assert_eq!(group_runs(&[1, 1, 2, 2, 2, 3]), vec![(1, 2), (2, 3), (3, 1)]);
    assert_eq!(group_runs(&[]), vec![]);
}

#[test]
fn test_indexed_filter() {
    let result = indexed_filter(&[10, 3, 20, 5, 30], |x| x >= 10);
    assert_eq!(result, vec![(0, 10), (2, 20), (4, 30)]);
}
