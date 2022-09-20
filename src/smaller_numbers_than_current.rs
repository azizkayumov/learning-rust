// https://leetcode.com/problems/how-many-numbers-are-smaller-than-the-current-number/

use core::num;

pub fn smaller_numbers_than_current(mut nums: Vec<i32>) -> Vec<i32> {
    // The input contains numbers constrained by: 0 <= nums[i] <= 100
    // Therefore, construct a counter array of size 100
    // Loop the input and increment each num[i]'s count
    let mut counter = [0; 101]; // including 100 itself
    for i in 0..nums.len() {
        let n = nums[i] as usize;
        counter[n] += 1;
    }
    // Build the result for each number from the sum of counters of smaller numbers
    for i in 1..counter.len() {
        counter[i] += counter[i - 1];
    }

    // reuse the nums vector to construct the result
    for i in 0..nums.len() {
        let n = nums[i] as usize;
        if n == 0 {
            nums[i] = 0;
        } else {
            nums[i] = counter[n - 1];
        }
    }

    nums
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
