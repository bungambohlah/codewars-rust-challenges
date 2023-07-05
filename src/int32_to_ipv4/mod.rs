pub fn run(int: u32) -> String {
    format!(
        "{}.{}.{}.{}",
        int >> 24,
        (int >> 16) & 255,
        (int >> 8) & 255,
        int & 255
    )
}

#[test]
fn basic() {
    assert_eq!(run(2154959208), "128.114.17.104");
    assert_eq!(run(2149583361), "128.32.10.1");
    assert_eq!(run(0), "0.0.0.0");
}
