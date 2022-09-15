// https://leetcode.com/problems/concatenation-of-array/
impl Solution {
    pub fn get_concatenation(nums: Vec<i32>) -> Vec<i32> {
        let n = nums.len();
        let mut res: Vec<i32> = Vec::with_capacity(2 * n); // create with 2n capacity
        res.extend(&nums);
        res.extend(&nums);
        return res;
    }
}
