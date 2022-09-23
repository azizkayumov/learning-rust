// https://leetcode.com/problems/decode-xored-array/

pub fn decode(mut encoded: Vec<i32>, first: i32) -> Vec<i32> {
    [].into()
}

#[test]
fn test_decode() {
    let expected: Vec<i32> = [1, 0, 2, 1].into();
    let input = [1, 2, 3].into();
    let output = decode(input, 1);
    assert_eq!(expected, output);
}
