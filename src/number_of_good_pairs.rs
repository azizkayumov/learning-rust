// https://leetcode.com/problems/number-of-good-pairs/

pub fn num_identical_pairs(nums: Vec<i32>) -> i32 {
    -1
}

#[test]
fn test_with_general_input() {
    let expected = 4;
    let input = [1, 2, 3, 1, 1, 3].into();
    let output = num_identical_pairs(input);
    assert_eq!(expected, output);
}

#[test]
fn test_with_only_ones() {
    let expected = 6;
    let input = [1, 1, 1, 1].into();
    let output = num_identical_pairs(input);
    assert_eq!(expected, output);
}

#[test]
fn test_with_increasing_array() {
    let expected = 0;
    let input = [1, 2, 3, 4].into();
    let output = num_identical_pairs(input);
    assert_eq!(expected, output);
}

#[test]
fn test_with_single_number() {
    let expected = 0;
    let input = [1].into();
    let output = num_identical_pairs(input);
    assert_eq!(expected, output);
}
