// https://leetcode.com/problems/count-items-matching-a-rule/

pub fn count_matches(items: Vec<Vec<String>>, rule_key: String, rule_value: String) -> i32 {
    -1
}

#[test]
fn test_count_matches() {
    let mut input: Vec<Vec<String>> = Vec::new();
    let mut item1 = Vec::new();
    item1.push(String::from("phone"));
    item1.push(String::from("blue"));
    item1.push(String::from("pixel"));
    input.push(item1);
    let mut item2 = Vec::new();
    item2.push(String::from("computer"));
    item2.push(String::from("silver"));
    item2.push(String::from("lenovo"));
    input.push(item2);
    let mut item3 = Vec::new();
    item3.push(String::from("phone"));
    item3.push(String::from("gold"));
    item3.push(String::from("iphone"));
    input.push(item3);
    let rule_key = String::from("color");
    let rule_value = String::from("silver");

    let expected = 1;
    let output = count_matches(input, rule_key, rule_value);
    assert_eq!(expected, output);
}
