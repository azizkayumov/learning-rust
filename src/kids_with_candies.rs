// https://leetcode.com/problems/kids-with-the-greatest-number-of-candies/

pub fn kids_with_candies(candies: Vec<i32>, extra_candies: i32) -> Vec<bool> {
    return [false].into();
}

#[test]
fn test_kids_with_candies() {
    let expected: Vec<bool> = [true, true, true, false, true].into();
    let input = [2, 3, 5, 1, 3].into();
    let extra_candies = 3;
    let output = kids_with_candies(input, extra_candies);
    assert_eq!(expected, output);
}

#[test]
fn test_with_one_extra_candy() {
    let expected: Vec<bool> = [true, false, false, false, false].into();
    let input = [4, 2, 1, 1, 2].into();
    let extra_candies = 1;
    let output = kids_with_candies(input, extra_candies);
    assert_eq!(expected, output);
}
