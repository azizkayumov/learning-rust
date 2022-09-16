// https://leetcode.com/problems/build-array-from-permutation/

pub fn build_array(mut nums: Vec<i32>) -> Vec<i32> {
    let n = nums.len();

    let N = n as i32;

    for i in 0..n {
        nums[i] += N * (nums[nums[i] as usize] % N);
    }
    for i in 0..n {
        nums[i] /= N;
    }

    return nums;
}

#[test]
fn test_build_array_from_permutation() {
    let input: Vec<i32> = [0, 2, 1, 5, 3, 4].into();
    let result: Vec<i32> = build_array(input);
    let expected: Vec<i32> = [0, 1, 2, 4, 5, 3].into();
    assert_eq!(result, expected);
}

#[test]
fn test_build_array_from_permutation_with_single_element() {
    let input: Vec<i32> = [0].into();
    let result: Vec<i32> = build_array(input);
    let expected: Vec<i32> = [0].into();
    assert_eq!(result, expected);
}

#[test]
fn test_build_array_from_permutation_no_order_change() {
    let input: Vec<i32> = [0, 1, 2, 3, 4].into();
    let result: Vec<i32> = build_array(input);
    let expected: Vec<i32> = [0, 1, 2, 3, 4].into();
    assert_eq!(result, expected);
}
