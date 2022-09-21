// https://leetcode.com/problems/defanging-an-ip-address/

pub fn defang_i_paddr(address: String) -> String {
    return String::from("");
}

#[test]
fn test_defang_ip_address() {
    let expected = String::from("1[.]1[.]1[.]1");
    let input = String::from("1.1.1.1");
    let output = defang_i_paddr(input);
    assert_eq!(expected, output);
}
