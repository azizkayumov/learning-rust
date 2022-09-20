// https://leetcode.com/problems/number-of-good-pairs/

pub fn num_identical_pairs(mut nums: Vec<i32>) -> i32 {
    nums.sort();

    // after sorting, count consecutive identical numbers
    // for each number: add the number of previously seen identical number to result:
    let mut result = 0;
    let mut cur_count_of_pairs = 0;
    for i in 1..nums.len() {
        if nums[i] == nums[i - 1] {
            cur_count_of_pairs += 1
        } else {
            cur_count_of_pairs = 0
        }
        result += cur_count_of_pairs;
    }
    result
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
