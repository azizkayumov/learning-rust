// https://leetcode.com/problems/maximum-number-of-words-found-in-sentences/

pub fn most_words_found(sentences: Vec<String>) -> i32 {
    -1
}

#[test]
fn test_most_words_founds() {
    let expected = 3;
    let mut input: Vec<String> = Vec::new();
    input.push(String::from("alice and bob love leetcode"));
    input.push(String::from("i think so too"));
    input.push(String::from("this is great thanks very much"));

    let output = most_words_found(input);
    assert_eq!(expected, output);
}
