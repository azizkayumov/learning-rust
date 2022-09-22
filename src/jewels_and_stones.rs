// https://leetcode.com/problems/jewels-and-stones/

pub fn num_jewels_in_stones(jewels: String, stones: String) -> i32 {
    let mut result = 0;
    result = stones.chars().filter(|&c| jewels.contains(c)).count();
    result as i32
}

#[test]
fn test_with_no_jewels() {
    let expected = 0;
    let input_jewels = String::from("z");
    let input_stones = String::from("ZZ");
    let output = num_jewels_in_stones(input_jewels, input_stones);
    assert_eq!(expected, output);
}

#[test]
fn test_with_jewels() {
    let expected = 1;
    let input_jewels = String::from("ngm");
    let input_stones = String::from("kxg");
    let output = num_jewels_in_stones(input_jewels, input_stones);
    assert_eq!(expected, output);
}
