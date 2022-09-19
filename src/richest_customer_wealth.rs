// https://leetcode.com/problems/richest-customer-wealth/

pub fn maximum_wealth(accounts: Vec<Vec<i32>>) -> i32 {
    let mut res = 0;
    for account in accounts {
        res = res.max(account.iter().sum());
    }
    return res;
}

#[test]
fn test_richest_customer_wealth() {
    let input = [[1, 2, 3].into(), [1, 2, 4].into(), [1, 2, 5].into()].into();
    let expected = 8;
    let output = maximum_wealth(input);
    assert_eq!(expected, output);
}
