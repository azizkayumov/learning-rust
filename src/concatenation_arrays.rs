// https://leetcode.com/problems/concatenation-of-array/
impl Solution {
    pub fn get_concatenation(mut nums: Vec<i32>) -> Vec<i32> {
        let n = nums.len();
        for i in 0..n {
            nums.push(nums[i]);
        }
        return nums;
    }
}
