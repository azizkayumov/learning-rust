// https://leetcode.com/problems/concatenation-of-array/
impl Solution {
    pub fn get_concatenation(mut nums: Vec<i32>) -> Vec<i32> {
        nums.extend_from_within(..);
        nums
    }
}
