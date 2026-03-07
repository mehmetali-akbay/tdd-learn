use iterators::*;

// =====================================================================
// Topic 1: Iterator Basics — Method Chains
// =====================================================================

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
fn test_flatten_with_empty_inner() {
    assert_eq!(
        flatten(&[vec![1], vec![], vec![2, 3]]),
        vec![1, 2, 3]
    );
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

#[test]
fn test_enumerate_format() {
    assert_eq!(
        enumerate_format(&["a", "b", "c"]),
        vec!["0: a", "1: b", "2: c"]
    );
    assert_eq!(enumerate_format(&[10, 20]), vec!["0: 10", "1: 20"]);
}

#[test]
fn test_enumerate_format_empty() {
    assert_eq!(enumerate_format::<i32>(&[]), Vec::<String>::new());
}

#[test]
fn test_zip_pairs() {
    assert_eq!(
        zip_pairs(&[1, 2, 3], &["a", "b", "c"]),
        vec![(1, "a"), (2, "b"), (3, "c")]
    );
    assert_eq!(zip_pairs(&[1, 2], &["a"]), vec![(1, "a")]);
}

#[test]
fn test_dot_product() {
    assert_eq!(dot_product(&[1, 2, 3], &[4, 5, 6]), 32); // 4+10+18
    assert_eq!(dot_product(&[], &[]), 0);
    assert_eq!(dot_product(&[1, 2], &[3]), 3); // only first pair
}

// =====================================================================
// Topic 2: Chained Transformations
// =====================================================================

#[test]
fn test_unique_words_sorted() {
    assert_eq!(
        unique_words_sorted("hello world Hello WORLD foo"),
        vec!["foo", "hello", "world"]
    );
}

#[test]
fn test_unique_words_sorted_empty() {
    assert_eq!(unique_words_sorted(""), Vec::<String>::new());
}

#[test]
fn test_take_while_ascending() {
    assert_eq!(take_while_ascending(&[1, 3, 5, 2, 7]), vec![1, 3, 5]);
    assert_eq!(take_while_ascending(&[5, 4, 3]), vec![5]);
    assert_eq!(take_while_ascending(&[]), Vec::<i32>::new());
}

#[test]
fn test_take_while_ascending_all() {
    assert_eq!(take_while_ascending(&[1, 2, 3, 4]), vec![1, 2, 3, 4]);
}

#[test]
fn test_skip_leading_zeros() {
    assert_eq!(skip_leading_zeros(&[0, 0, 0, 1, 2, 0, 3]), vec![1, 2, 0, 3]);
    assert_eq!(skip_leading_zeros(&[1, 2, 3]), vec![1, 2, 3]);
    assert_eq!(skip_leading_zeros(&[0, 0, 0]), Vec::<i32>::new());
}

#[test]
fn test_group_consecutive() {
    assert_eq!(
        group_consecutive(&[1, 1, 2, 2, 2, 3]),
        vec![vec![1, 1], vec![2, 2, 2], vec![3]]
    );
    assert_eq!(group_consecutive(&[]), Vec::<Vec<i32>>::new());
}

#[test]
fn test_group_consecutive_single() {
    assert_eq!(group_consecutive(&[5]), vec![vec![5]]);
}

#[test]
fn test_pairwise_diff() {
    assert_eq!(pairwise_diff(&[1, 3, 6, 10]), vec![2, 3, 4]);
    assert_eq!(pairwise_diff(&[5]), Vec::<i32>::new());
    assert_eq!(pairwise_diff(&[]), Vec::<i32>::new());
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
fn test_every_nth() {
    assert_eq!(every_nth(&[1, 2, 3, 4, 5, 6], 2), vec![1, 3, 5]);
    assert_eq!(every_nth(&[1, 2, 3], 1), vec![1, 2, 3]);
    assert_eq!(every_nth(&[1, 2, 3], 0), Vec::<i32>::new());
}

#[test]
fn test_flatten_and_sort() {
    assert_eq!(
        flatten_and_sort(&[vec![3, 1], vec![2, 5], vec![4]]),
        vec![1, 2, 3, 4, 5]
    );
    assert_eq!(flatten_and_sort(&[]), Vec::<i32>::new());
}

// =====================================================================
// Topic 3: Custom Iterator — Counter
// =====================================================================

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

#[test]
fn test_counter_map_and_collect() {
    let squares: Vec<i32> = Counter::with_limit(1, 1, 6).map(|x| x * x).collect();
    assert_eq!(squares, vec![1, 4, 9, 16, 25]);
}

// =====================================================================
// Topic 4: Fibonacci & Sequences
// =====================================================================

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
    let even_fibs: Vec<u64> = Fibonacci::new()
        .take_while(|&x| x < 100)
        .filter(|x| x % 2 == 0)
        .collect();
    assert_eq!(even_fibs, vec![0, 2, 8, 34]);
}

#[test]
fn test_collatz_sequence() {
    assert_eq!(collatz_sequence(6), vec![6, 3, 10, 5, 16, 8, 4, 2, 1]);
    assert_eq!(collatz_sequence(1), vec![1]);
    assert_eq!(collatz_sequence(0), Vec::<u64>::new());
}

#[test]
fn test_collatz_length() {
    assert_eq!(collatz_length(6), 9);
    assert_eq!(collatz_length(1), 1);
    assert_eq!(collatz_length(27), 112);
}

#[test]
fn test_collatz_sequence_powers_of_two() {
    assert_eq!(collatz_sequence(8), vec![8, 4, 2, 1]);
    assert_eq!(collatz_sequence(16), vec![16, 8, 4, 2, 1]);
}

// =====================================================================
// Topic 5: Chunks & Adapters
// =====================================================================

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
    let words = vec!["a".to_string(), "b".to_string(), "c".to_string(), "d".to_string()];
    let chunks: Vec<Vec<String>> = Chunks::new(words, 2).collect();
    assert_eq!(chunks.len(), 2);
    assert_eq!(chunks[0], vec!["a", "b"]);
    assert_eq!(chunks[1], vec!["c", "d"]);
}

#[test]
fn test_cycle_n() {
    assert_eq!(cycle_n(&[1, 2, 3], 2), vec![1, 2, 3, 1, 2, 3]);
    assert_eq!(cycle_n(&[1, 2], 0), Vec::<i32>::new());
    assert_eq!(cycle_n::<i32>(&[], 5), Vec::<i32>::new());
}

#[test]
fn test_cycle_n_single() {
    assert_eq!(cycle_n(&[42], 3), vec![42, 42, 42]);
}

#[test]
fn test_step_by_collect() {
    assert_eq!(step_by_collect(vec![1, 2, 3, 4, 5, 6], 2), vec![1, 3, 5]);
    assert_eq!(step_by_collect(vec![1, 2, 3, 4, 5, 6], 3), vec![1, 4]);
}

#[test]
fn test_step_by_collect_step_one() {
    assert_eq!(step_by_collect(vec![1, 2, 3], 1), vec![1, 2, 3]);
}

// =====================================================================
// Topic 6: Advanced — Iterator Combinators
// =====================================================================

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
fn test_interleave_empty() {
    assert_eq!(interleave::<i32>(&[], &[1, 2]), vec![1, 2]);
    assert_eq!(interleave::<i32>(&[], &[]), Vec::<i32>::new());
}

#[test]
fn test_chain_all() {
    assert_eq!(
        chain_all(vec![vec![1, 2], vec![3], vec![4, 5]]),
        vec![1, 2, 3, 4, 5]
    );
    assert_eq!(chain_all(vec![]), Vec::<i32>::new());
}

#[test]
fn test_unique_from_both() {
    let result = unique_from_both(&[1, 2, 3], &[2, 3, 4]);
    assert_eq!(result, vec![1, 2, 3, 4]);
}

#[test]
fn test_unique_from_both_no_overlap() {
    assert_eq!(unique_from_both(&[1, 2], &[3, 4]), vec![1, 2, 3, 4]);
}

#[test]
fn test_cumulative_max() {
    assert_eq!(cumulative_max(&[1, 3, 2, 5, 4]), vec![1, 3, 3, 5, 5]);
    assert_eq!(cumulative_max(&[]), Vec::<i32>::new());
}

#[test]
fn test_group_runs() {
    assert_eq!(
        group_runs(&[1, 1, 2, 2, 2, 3]),
        vec![(1, 2), (2, 3), (3, 1)]
    );
    assert_eq!(group_runs(&[]), vec![]);
}

#[test]
fn test_group_runs_no_repeats() {
    assert_eq!(
        group_runs(&[1, 2, 3]),
        vec![(1, 1), (2, 1), (3, 1)]
    );
}

#[test]
fn test_indexed_filter() {
    let result = indexed_filter(&[10, 3, 20, 5, 30], |x| x >= 10);
    assert_eq!(result, vec![(0, 10), (2, 20), (4, 30)]);
}

#[test]
fn test_indexed_filter_none() {
    assert_eq!(indexed_filter(&[1, 2, 3], |x| x > 10), vec![]);
}

#[test]
fn test_frequency_count() {
    let freq = frequency_count(&[1, 2, 2, 3, 3, 3]);
    assert_eq!(freq[&1], 1);
    assert_eq!(freq[&2], 2);
    assert_eq!(freq[&3], 3);
}

#[test]
fn test_frequency_count_strings() {
    let freq = frequency_count(&["a", "b", "a", "a"]);
    assert_eq!(freq[&"a"], 3);
    assert_eq!(freq[&"b"], 1);
}

#[test]
fn test_top_n_frequent() {
    let result = top_n_frequent(&[1, 2, 2, 3, 3, 3, 4], 2);
    assert_eq!(result[0], (3, 3));
    assert_eq!(result[1], (2, 2));
}

#[test]
fn test_top_n_frequent_more_than_unique() {
    let result = top_n_frequent(&[1, 2, 3], 10);
    assert_eq!(result.len(), 3);
}

// =====================================================================
// Topic 7: Iterator Utilities & Conversions
// =====================================================================

#[test]
fn test_batch_process() {
    let results = batch_process(&[1, 2, 3, 4, 5], 2, |batch| {
        batch.iter().sum::<i32>()
    });
    assert_eq!(results, vec![3, 7, 5]); // [1+2, 3+4, 5]
}

#[test]
fn test_batch_process_single() {
    let results = batch_process(&[1, 2, 3], 1, |batch| batch.len());
    assert_eq!(results, vec![1, 1, 1]);
}

#[test]
fn test_partition_by() {
    let (evens, odds) = partition_by(&[1, 2, 3, 4, 5], |x| x % 2 == 0);
    assert_eq!(evens, vec![2, 4]);
    assert_eq!(odds, vec![1, 3, 5]);
}

#[test]
fn test_partition_by_all_match() {
    let (pass, fail) = partition_by(&[2, 4, 6], |x| x % 2 == 0);
    assert_eq!(pass, vec![2, 4, 6]);
    assert!(fail.is_empty());
}

#[test]
fn test_filter_map_vec() {
    let result = filter_map_vec(&["1", "abc", "3", "def"], |s| s.parse::<i32>().ok());
    assert_eq!(result, vec![1, 3]);
}

#[test]
fn test_filter_map_vec_all_none() {
    let result = filter_map_vec(&["a", "b"], |s| s.parse::<i32>().ok());
    assert!(result.is_empty());
}

#[test]
fn test_parse_all_numbers() {
    assert_eq!(
        parse_all_numbers(&["1.5", "2.0", "3.14"]),
        Ok(vec![1.5, 2.0, 3.14])
    );
    let err = parse_all_numbers(&["1", "abc", "3"]);
    assert!(err.is_err());
    assert_eq!(err.unwrap_err(), "cannot parse 'abc'");
}

#[test]
fn test_parse_all_numbers_empty() {
    assert_eq!(parse_all_numbers(&[]), Ok(vec![]));
}

#[test]
fn test_max_with_index() {
    assert_eq!(max_with_index(&[3, 7, 1, 9, 2]), Some((3, 9)));
    assert_eq!(max_with_index(&[42]), Some((0, 42)));
    assert_eq!(max_with_index(&[]), None);
}

#[test]
fn test_min_with_index() {
    assert_eq!(min_with_index(&[3, 7, 1, 9, 2]), Some((2, 1)));
    assert_eq!(min_with_index(&[]), None);
}

#[test]
fn test_join_with() {
    assert_eq!(join_with(&[1, 2, 3], ", "), "1, 2, 3");
    assert_eq!(join_with(&["a", "b", "c"], "-"), "a-b-c");
    assert_eq!(join_with::<i32>(&[], ", "), "");
}

#[test]
fn test_histogram() {
    let hist = histogram(&[1, 3, 5, 7, 11, 15], 5);
    // 1=>0, 3=>0, 5=>5, 7=>5, 11=>10, 15=>15
    assert_eq!(hist, vec![(0, 2), (5, 2), (10, 1), (15, 1)]);
}

#[test]
fn test_histogram_empty() {
    assert_eq!(histogram(&[], 5), Vec::<(i32, usize)>::new());
    assert_eq!(histogram(&[1, 2], 0), Vec::<(i32, usize)>::new());
}

#[test]
fn test_running_average() {
    let avgs = running_average(&[2.0, 4.0, 6.0]);
    assert!((avgs[0] - 2.0).abs() < 0.001); // 2/1
    assert!((avgs[1] - 3.0).abs() < 0.001); // 6/2
    assert!((avgs[2] - 4.0).abs() < 0.001); // 12/3
}

#[test]
fn test_running_average_empty() {
    assert_eq!(running_average(&[]), Vec::<f64>::new());
}

#[test]
fn test_unzip_pairs() {
    let (a, b) = unzip_pairs(&[(1, "a"), (2, "b"), (3, "c")]);
    assert_eq!(a, vec![1, 2, 3]);
    assert_eq!(b, vec!["a", "b", "c"]);
}

#[test]
fn test_unzip_pairs_empty() {
    let (a, b) = unzip_pairs::<i32, i32>(&[]);
    assert!(a.is_empty());
    assert!(b.is_empty());
}

// =====================================================================
// Integration / Cross-topic tests
// =====================================================================

#[test]
fn test_counter_then_chunk() {
    let nums: Vec<i32> = Counter::with_limit(0, 1, 10).collect();
    let chunks = chunk_slice(&nums, 3);
    assert_eq!(
        chunks,
        vec![vec![0, 1, 2], vec![3, 4, 5], vec![6, 7, 8], vec![9]]
    );
}

#[test]
fn test_fib_then_frequency() {
    let fibs = first_n_fib(10);
    let freq = frequency_count(&fibs);
    assert_eq!(freq[&1], 2); // 1 appears twice in first 10 fibs
    assert_eq!(freq[&0], 1);
}

#[test]
fn test_running_sum_and_diff() {
    let sums = running_sum(&[1, 2, 3, 4]);
    let diffs = pairwise_diff(&sums);
    // running_sum = [1, 3, 6, 10], diffs = [2, 3, 4]
    assert_eq!(diffs, vec![2, 3, 4]);
}

#[test]
fn test_batch_and_flatten() {
    let items = vec![1, 2, 3, 4, 5, 6];
    let chunks = chunk_slice(&items, 2);
    let flat = flatten(&chunks);
    assert_eq!(flat, items);
}
