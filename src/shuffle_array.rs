// https://leetcode.com/problems/shuffle-an-array/

use rand::Rng;

struct Solution {
    original: Vec<i32>,
}

impl Solution {
    fn new(mut nums: Vec<i32>) -> Self {
        Solution { original: nums }
    }

    fn reset(&self) -> Vec<i32> {
        return self.original.clone();
    }

    fn shuffle(&self) -> Vec<i32> {
        let mut shuffled = self.original.clone();
        let mut range = rand::thread_rng();
        for i in 0..shuffled.len() {
            shuffled.swap(i, range.gen_range(i..self.original.len()));
        }
        return shuffled;
    }
}

#[test]
fn test_shuffle_array() {
    let expected: Vec<i32> = [1, 2, 3].into();
    let solution = Solution::new([1, 2, 3].into());
    let shuffled = solution.shuffle();
    let reset = solution.reset();
    assert_eq!(expected, reset);
}
