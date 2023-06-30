pub fn run(isbn: &str) -> bool {
    let isbn_chars = isbn.chars();

    // check length of isbn is 10
    if isbn_chars.clone().count() != 10 {
        return false;
    }

    let mut sum = 0;
    for (i, char) in isbn_chars.enumerate() {
        // The first nine characters are digits 0-9. The last digit can be 0-9 or X, to indicate a value of 10.
        let mut digit = char.to_digit(10).unwrap_or(0);
        if i == 9 && char == 'X' {
            digit = 10;
        }

        if digit > 0 {
            sum += digit * (i + 1) as u32;
        }
    }

    sum > 0 && sum % 11 == 0

    // BEST PRACTICE OF CODE
    // isbn.len() == 10 &&
    // isbn.chars().enumerate().all(|(i, c)| c.is_numeric() || (c == 'X' && i == 9)) &&
    // isbn.chars().enumerate().map(|(i, c)| c.to_digit(10).unwrap_or(10) * (i as u32 + 1)).sum::<u32>() % 11 == 0
}

// Add your tests here.
// See https://doc.rust-lang.org/stable/rust-by-example/testing/unit_testing.html

pub fn dotest(isbn: &str, expected: bool) {
    let actual = run(isbn);
    assert!(
        actual == expected,
        "Test failed with isbn = {isbn}\nExpected {expected} but got {actual}"
    )
}

#[test]
fn sample_tests() {
    dotest("1112223339", true);
    dotest("048665088X", true);
    dotest("1293000000", true);
    dotest("1234554321", true);
    dotest("1234512345", false);
    dotest("1293", false);
    dotest("X123456788", false);
    dotest("ABCDEFGHIJ", false);
    dotest("XXXXXXXXXX", false);
    dotest("123456789T", false);
}
