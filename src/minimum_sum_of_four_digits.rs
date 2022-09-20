// https://leetcode.com/problems/minimum-sum-of-four-digit-number-after-splitting-digits/

pub fn minimum_sum(num: i32) -> i32 {
    -1 // failing test case
}

#[test]
fn test_with_non_zero_digits() {
    let expected = 52;
    let input = 2932;
    let output = minimum_sum(input);
    assert_eq!(expected, output);
}

#[test]
fn test_with_some_zero_digits() {
    let expected = 13;
    let input = 4009;
    let output = minimum_sum(input);
    assert_eq!(expected, output);
}

#[test]
fn test_with_largest_input() {
    let expected = 36;
    let input = 9999;
    let output = minimum_sum(input);
    assert_eq!(expected, output);
}

#[test]
fn test_with_smallest_input() {
    let expected = 1;
    let input = 1000;
    let output = minimum_sum(input);
    assert_eq!(expected, output);
}
