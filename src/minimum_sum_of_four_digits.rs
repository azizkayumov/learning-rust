// https://leetcode.com/problems/minimum-sum-of-four-digit-number-after-splitting-digits/

pub fn minimum_sum(num: i32) -> i32 {
    let mut result = 0;
    // Extract digits: 1234 => d1 d2 d3 d4 = 1, 2, 3, 4
    let d1 = num / 1000;
    let d2 = (num % 1000) / 100;
    let d3 = (num % 100) / 10;
    let d4 = num % 10;
    // Form two numbers with both having 2 digits: x and y
    // Then, the first digits of x and y are multiplied by 10 and added to the result
    // and the last digits of x and y are also added to the result

    // To minimize the sum, place 2 smallest numbers as first digits in x and y
    // and then the largest digits as second digits in x and y
    let mut digits: Vec<i32> = [d1, d2, d3, d4].into();
    digits.sort();
    result += digits.pop().unwrap() + digits.pop().unwrap();
    result += digits.pop().unwrap() * 10 + digits.pop().unwrap() * 10;
    return result;
}

#[test]
fn test_with_non_zero_digits() {
    let expected = 52;
    let input = 2932;
    let output = minimum_sum(input);
    assert_eq!(expected, output);
}

#[test]
fn test_with_some_zero_digits() {
    let expected = 13;
    let input = 4009;
    let output = minimum_sum(input);
    assert_eq!(expected, output);
}

#[test]
fn test_with_largest_input() {
    let expected = 198;
    let input = 9999;
    let output = minimum_sum(input);
    assert_eq!(expected, output);
}

#[test]
fn test_with_smallest_input() {
    let expected = 1;
    let input = 1000;
    let output = minimum_sum(input);
    assert_eq!(expected, output);
}
