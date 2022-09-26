// https://leetcode.com/problems/count-of-matches-in-tournament/

pub fn number_of_matches(mut n: i32) -> i32 {
    let mut result = 0;
    let mut remainder = 0;
    while n > 0 {
        n += remainder;
        remainder = n % 2;
        n /= 2;
        result += n;
    }
    result
}

#[test]
fn test_number_of_matches() {
    let expected = 6;
    let input = 7;
    let output = number_of_matches(input);
    assert_eq!(expected, output);
}
