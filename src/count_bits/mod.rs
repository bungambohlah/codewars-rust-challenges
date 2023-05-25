pub fn run(n: i64) -> u32 {
    n.count_ones()
}

#[test]
pub fn returns_expected() {
    assert_eq!(run(0), 0);
    assert_eq!(run(4), 1);
    assert_eq!(run(7), 3);
    assert_eq!(run(9), 2);
    assert_eq!(run(10), 2);
}
