// https://leetcode.com/problems/final-value-of-variable-after-performing-operations/

pub fn final_value_after_operations(operations: Vec<String>) -> i32 {
    operations
        .iter()
        .map(|op| if op.contains("++") { 1 } else { -1 })
        .sum()
}

#[test]
fn test_final_value_after_increment() {
    let expected = 2;
    let input = vec!["X++".into(), "++X".into()];
    let output = final_value_after_operations(input);
    assert_eq!(expected, output);
}

#[test]
fn test_final_value_after_decrement() {
    let expected = -2;
    let input = vec!["X--".into(), "--X".into()];
    let output = final_value_after_operations(input);
    assert_eq!(expected, output);
}

#[test]
fn test_final_value_after_operations() {
    let expected = 3;
    let input = vec![
        "X++".into(),
        "--X".into(),
        "++X".into(),
        "++X".into(),
        "X++".into(),
    ];
    let output = final_value_after_operations(input);
    assert_eq!(expected, output);
}
