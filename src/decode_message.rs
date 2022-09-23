// https://leetcode.com/problems/decode-the-message/

pub fn decode_message(key: String, message: String) -> String {
    String::from("")
}

#[test]
fn test_decode_message() {
    let expected = String::from("this is a secret");
    let input_key = String::from("the quick brown fox jumps over the lazy dog");
    let input_message = String::from("vkbs bs t suepuv");
    let output = decode_message(input_key, input_message);
    assert_eq!(expected, output);
}
