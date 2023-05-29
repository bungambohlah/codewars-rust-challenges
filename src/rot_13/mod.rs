pub fn run(message: &str) -> String {
    message
        .chars()
        .map(|c| {
            if c.is_ascii_alphabetic() {
                if c.is_ascii_uppercase() {
                    (((c as u8 - b'A') + 13) % 26 + 'A' as u8) as char
                } else {
                    (((c as u8 - b'a') + 13) % 26 + 'a' as u8) as char
                }
            } else {
                c
            }
        })
        .collect::<String>()

    // BEST PRACTICE
    // message.chars().map(|c| {
    //         match c {
    //             'A' ..= 'M' | 'a' ..= 'm' => ((c as u8) + 13) as char,
    //             'N' ..= 'Z' | 'n' ..= 'z' => ((c as u8) - 13) as char,
    //             _ => c,
    //         }
    //     }).collect()
}

const ERR_MSG: &str = "\nYour result (left) did not match the expected output (right)";
pub fn dotest(s: &str, expected: &str) {
    assert_eq!(run(s), expected, "{ERR_MSG} with message = \"{s}\"")
}

#[test]
fn sample_tests() {
    dotest("test", "grfg");
    dotest("Test", "Grfg");
}
