// https://leetcode.com/problems/how-many-numbers-are-smaller-than-the-current-number/

pub fn smaller_numbers_than_current(nums: Vec<i32>) -> Vec<i32> {
    [0].into()
}

#[test]
fn test_with_general_input() {
    let expected: Vec<i32> = [4, 0, 1, 1, 3].into();
    let input = [8, 1, 2, 2, 3].into();
    let output = smaller_numbers_than_current(input);
    assert_eq!(expected, output);
}

#[test]
fn test_with_increasing_array() {
    let expected: Vec<i32> = [0, 1, 2, 3].into();
    let input = [1, 2, 3, 4].into();
    let output = smaller_numbers_than_current(input);
    assert_eq!(expected, output);
}
