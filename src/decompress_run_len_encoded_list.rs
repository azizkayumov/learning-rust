// https://leetcode.com/problems/decompress-run-length-encoded-list/

pub fn decompress_rl_elist(nums: Vec<i32>) -> Vec<i32> {
    [].into()
}

#[test]
fn test_decompress_rl_elist() {
    let expected: Vec<i32> = [2, 4, 4, 4].into();
    let input = [1, 2, 3, 4].into();
    let output = decompress_rl_elist(input);
    assert_eq!(expected, output);
}
