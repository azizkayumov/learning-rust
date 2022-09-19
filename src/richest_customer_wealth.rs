// https://leetcode.com/problems/richest-customer-wealth/

pub fn maximum_wealth(accounts: Vec<Vec<i32>>) -> i32 {
    accounts
        .iter()
        .map(|account| account.iter().sum())
        .max()
        .unwrap()
}

#[test]
fn test_richest_customer_wealth() {
    let input = [[1, 2, 3].into(), [1, 2, 4].into(), [1, 2, 5].into()].into();
    let expected = 8;
    let output = maximum_wealth(input);
    assert_eq!(expected, output);
}
