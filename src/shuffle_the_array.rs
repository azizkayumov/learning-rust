// https://leetcode.com/problems/shuffle-the-array/

pub fn shuffle(nums: Vec<i32>, n: i32) -> Vec<i32> {
    let mut result: Vec<i32> = Vec::new();
    for i in 0..n as usize {
        result.push(nums[i]);
        result.push(nums[i + n as usize]);
    }
    result
}

#[test]
fn test_shuffle_array() {
    let input = [1, 2, 3, 4, 4, 3, 2, 1].into();
    let expected: Vec<i32> = [1, 4, 2, 3, 3, 2, 4, 1].into();
    let output = shuffle(input, 4);
    assert_eq!(expected, output);
}
