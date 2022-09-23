// https://leetcode.com/problems/sorting-the-sentence/

pub fn sort_sentence(s: String) -> String {
    // Split the input by a space, store it in a vector
    let mut words: Vec<&str> = s.split(' ').collect();

    // Sort the vector by the last char of each word
    words.sort_by(|&a, &b| a.chars().last().unwrap().cmp(&b.chars().last().unwrap()));

    // construct the result not including the last digit char of each word
    let mut res = String::from("");
    for i in 0..words.len() {
        let word = words[i];
        res += &word[0..(word.len() - 1)];
        if i != words.len() - 1 {
            res += " ";
        }
    }

    res
}

#[test]
fn test_sorting_sentence() {
    let expected = String::from("This is a sentence");
    let input = String::from("is2 sentence4 This1 a3");
    let output = sort_sentence(input);
    assert_eq!(expected, output);
}
