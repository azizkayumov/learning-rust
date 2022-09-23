// https://leetcode.com/problems/decode-the-message/
use std::collections::HashMap;

pub fn decode_message(key: String, message: String) -> String {
    let mut table: HashMap<char,char> = HashMap::new();
    let mut letter = 'a' as u8;

    for c in key.chars() {
        if c.is_alphabetic() && !table.contains_key(&c) {
            table.insert(c, letter as char);
            letter += 1;
        }
    }

    let mut result = String::with_capacity(message.len());
    for c in message.chars() {
        if c.is_alphabetic() {
            result.push(table[&c]);
        }else{
            result.push(c);
        }
    }

    result
}

#[test]
fn test_decode_message() {
    let expected = String::from("this is a secret");
    let input_key = String::from("the quick brown fox jumps over the lazy dog");
    let input_message = String::from("vkbs bs t suepuv");
    let output = decode_message(input_key, input_message);
    assert_eq!(expected, output);
}
