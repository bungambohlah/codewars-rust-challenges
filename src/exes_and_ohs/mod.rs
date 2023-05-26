pub fn run(string: &'static str) -> bool {
    string
        .chars()
        .filter(|&c| {
            // lowercase c and compare with 'x'
            match c.to_lowercase().last() {
                Some('x') => true,
                _ => false,
            }
        })
        .count()
        == string
            .chars()
            .filter(|&c| {
                // lowercase c and compare with 'x'
                match c.to_lowercase().last() {
                    Some('o') => true,
                    _ => false,
                }
            })
            .count()
}

#[test]
pub fn returns_expected() {
    assert_eq!(run("xo"), true);
    assert_eq!(run("Xo"), true);
    assert_eq!(run("xxOo"), true);
    assert_eq!(run("xxxm"), false);
    assert_eq!(run("Oo"), false);
    assert_eq!(run("ooom"), false);
}
