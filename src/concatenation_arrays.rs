// https://leetcode.com/problems/concatenation-of-array/

pub fn get_concatenation(mut nums: Vec<i32>) -> Vec<i32> {
    nums.extend_from_within(..);
    nums
}

#[test]
fn test_get_concatenation() {
    let input: Vec<i32> = [1, 2, 3, 4, 5].into();
    let expected: Vec<i32> = [1, 2, 3, 4, 5, 1, 2, 3, 4, 5].into();
    let result = get_concatenation(input);
    assert_eq!(expected, result);
}
