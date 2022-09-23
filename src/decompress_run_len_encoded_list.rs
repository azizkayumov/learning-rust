// https://leetcode.com/problems/decompress-run-length-encoded-list/

pub fn decompress_rl_elist(nums: Vec<i32>) -> Vec<i32> {
    let mut res: Vec<i32> = Vec::new();
    let mut i = 0;
    while i < nums.len() {
        let size = res.len();
        let frequency = nums[i];
        let value = nums[i + 1];
        // Vec::resize() => it extends the vector if its new size is larger than prev.size and fills up with value:
        res.resize(size + frequency as usize, value);
        i += 2;
    }
    res
}

#[test]
fn test_decompress_rl_elist() {
    let expected: Vec<i32> = [2, 4, 4, 4].into();
    let input = [1, 2, 3, 4].into();
    let output = decompress_rl_elist(input);
    assert_eq!(expected, output);
}
