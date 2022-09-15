// https://leetcode.com/problems/add-two-integers/

impl Solution {
    pub fn sum(num1: i32, num2: i32) -> i32 {
        return num1 + num2; // this does not overflow i32 because of the constraints: -100<= nums1,nums2 <= 100
    }
}
