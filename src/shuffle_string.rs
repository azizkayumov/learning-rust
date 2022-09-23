// https://leetcode.com/problems/shuffle-string/

pub fn restore_string(s: String, indices: Vec<i32>) -> String {
    String::from("")
}

#[test]
fn test_restore_string() {
    let expected = String::from("leetcode");
    let input = String::from("codeleet");
    let input_indices = [4, 5, 6, 7, 0, 2, 1, 3].into();
    let output = restore_string(input, input_indices);
    assert_eq!(expected, output);
}

#[test]
fn test_with_same_output() {
    let expected = String::from("abc");
    let input = String::from("abc");
    let input_indices = [0, 1, 2].into();
    let output = restore_string(input, input_indices);
    assert_eq!(expected, output);
}
