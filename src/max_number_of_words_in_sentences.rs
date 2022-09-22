// https://leetcode.com/problems/maximum-number-of-words-found-in-sentences/

pub fn most_words_found(sentences: Vec<String>) -> i32 {
    // Assuming that a space char is not followed by another space char, and
    // each sentences is trimmed (removed spaces from both sides), then:
    // the number of words = the number of spaces + 1
    sentences
        .iter()
        .map(|sentence| sentence.matches(" ").count() + 1)
        .max()
        .unwrap() as i32
}

#[test]
fn test_most_words_founds() {
    let expected = 6;
    let mut input: Vec<String> = Vec::new();
    input.push(String::from("alice and bob love leetcode"));
    input.push(String::from("i think so too"));
    input.push(String::from("this is great thanks very much"));

    let output = most_words_found(input);
    assert_eq!(expected, output);
}
