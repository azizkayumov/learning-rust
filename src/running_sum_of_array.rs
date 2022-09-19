// https://leetcode.com/problems/running-sum-of-1d-array/

pub fn running_sum(mut nums: Vec<i32>) -> Vec<i32> {
    for i in 1..nums.len() {
        nums[i] += nums[i - 1];
    }
    nums
}

#[test]
fn test_running_sum() {
    let input: Vec<i32> = [1, 2, 3, 4, 5].into();
    let expected: Vec<i32> = [1, 3, 6, 10, 15].into();
    let output = running_sum(input);
    assert_eq!(expected, output);
}

#[test]
fn test_running_sum_with_one_number() {
    let input: Vec<i32> = [1].into();
    let expected: Vec<i32> = [1].into();
    let output: Vec<i32> = running_sum(input);
    assert_eq!(expected, output);
}
