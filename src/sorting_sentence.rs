// https://leetcode.com/problems/sorting-the-sentence/

pub fn sort_sentence(s: String) -> String {
    String::from("-1")
}

#[test]
fn test_sorting_sentence() {
    let expected = String::from("This is a sentence");
    let input = String::from("is2 sentence4 This1 a3");
    let output = sort_sentence(input);
    assert_eq!(expected, output);
}
