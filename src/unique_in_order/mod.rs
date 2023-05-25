pub fn run<T>(sequence: T) -> Vec<T::Item>
where
    T: std::iter::IntoIterator,
    T::Item: std::cmp::PartialEq + std::fmt::Debug,
{
    let mut result = Vec::new();
    let mut seq_iter = sequence.into_iter();
    while let Some(v) = seq_iter.next() {
        match result.last() {
            Some(y) => {
                if *y != v {
                    result.push(v)
                }
            }
            None => result.push(v),
        }
    }
    result

    // OR JUST USE dedup function from Iteration trait, example:
    // let mut v: Vec<_> = sequence.into_iter().collect();
    // v.dedup();
    // v
}

#[test]
fn sample_test() {
    assert_eq!(
        run("AAAABBBCCDAABBB".chars()),
        vec!['A', 'B', 'C', 'D', 'A', 'B']
    );
}
