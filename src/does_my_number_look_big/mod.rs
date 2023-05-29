pub fn run(num: u64) -> bool {
    num.to_string()
        .chars()
        .map(|c| c.to_digit(10).unwrap() as u64)
        .map(|d| d.pow(num.to_string().len() as u32))
        .sum::<u64>()
        == num
}

pub fn dotest(input: u64, expected: bool) {
    let actual = run(input);
    assert_eq!(
        actual, expected,
        "\nIncorrect answer for n={}\nExpected: {expected}\nActual: {actual}",
        input
    )
}

#[test]
fn basic_tests() {
    dotest(7, true);
    dotest(371, true);
    dotest(122, false);
    dotest(4887, false);
}
