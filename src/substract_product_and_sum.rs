// https://leetcode.com/problems/subtract-the-product-and-sum-of-digits-of-an-integer/

pub fn subtract_product_and_sum(n: i32) -> i32 {
    let mut sum = 0;
    let mut product = 1;

    // Extracting ith digit (starting from right) of n:
    // digit[i] = n % 10.pow(i+1) / 10.pow(i)
    // Since we need the sum and product of digits, we don't have to store all digits,
    // but update the sum and the product until the 10.pow(i) > n

    let mut divider = 1; // 10^0
    let mut modder = 10; // 10^1
    while divider <= n {
        let digit: i32 = n % modder / divider;
        sum += digit;
        product *= digit;
        divider *= 10;
        modder *= 10;
    }
    product - sum
}

#[test]
fn test_product_and_sum() {
    let expected = 15;
    let input = 234;
    let output = subtract_product_and_sum(input);
    assert_eq!(expected, output);
}

#[test]
fn test_product_and_sum_with_ten_power() {
    let expected = -1;
    let input = 10000;
    let output = subtract_product_and_sum(input);
    assert_eq!(expected, output);
}
