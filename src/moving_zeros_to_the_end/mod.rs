pub fn run(arr: &[u8]) -> Vec<u8> {
    let count_zeros = arr.iter().filter(|&&x| x == 0).count();
    let mut result_list = arr
        .to_vec()
        .into_iter()
        .filter(|x| *x != 0)
        .collect::<Vec<u8>>();
    for _ in 0..count_zeros {
        result_list.push(0);
    }
    result_list

    // BEST PRACTICE
    // let mut ys = Vec::with_capacity(xs.len());
    // ys.extend(xs.iter().filter(|&&x| x != 0));
    // ys.resize(xs.len(), 0);
    // ys
}

#[cfg(test)]

pub fn dotest(a: &[u8], expected: &[u8]) {
    let actual = run(a);
    assert!(
        actual == expected,
        "With arr = {a:?}\nExpected {expected:?} but got {actual:?}"
    )
}

#[test]
pub fn sample_tests() {
    dotest(
        &[1, 2, 0, 1, 0, 1, 0, 3, 0, 1],
        &[1, 2, 1, 1, 3, 1, 0, 0, 0, 0],
    );
    dotest(
        &[9, 0, 0, 9, 1, 2, 0, 1, 0, 1, 0, 3, 0, 1, 9, 0, 0, 0, 0, 9],
        &[9, 9, 1, 2, 1, 1, 3, 1, 9, 9, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
    );
    dotest(&[0, 0], &[0, 0]);
    dotest(&[0], &[0]);
    dotest(&[], &[]);
}
