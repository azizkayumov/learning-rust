// https://leetcode.com/problems/number-of-steps-to-reduce-a-number-to-zero/

pub fn number_of_steps(num: i32) -> i32 {
    -1
}

#[test]
fn test_with_even_number() {
    let expected = 6;
    let input = 14;
    let output = number_of_steps(input);
    assert_eq!(expected, output);
}

#[test]
fn test_with_odd_number() {
    let expected = 5;
    let input = 7;
    let output = number_of_steps(input);
    assert_eq!(expected, output);
}
