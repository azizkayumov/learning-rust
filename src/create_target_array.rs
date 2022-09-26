// https://leetcode.com/problems/create-target-array-in-the-given-order/

pub fn create_target_array(nums: Vec<i32>, index: Vec<i32>) -> Vec<i32> {
    [].into()
}

#[test]
fn test_create_target_array() {
    let expected: Vec<i32> = [0, 4, 1, 3, 2].into();
    let input_nums = [0, 1, 2, 3, 4].into();
    let input_index = [0, 1, 2, 2, 1].into();
    let output = create_target_array(input_nums, input_index);
    assert_eq!(expected, output);
}
