pub fn run(n: i64, p: i32) -> i64 {
    let mut p = p;
    let digits: Vec<i64> = n
        .to_string()
        .chars()
        .map(|c| c.to_digit(10).unwrap() as i64)
        .collect();

    let mut count = 0;
    for i in 0..digits.len() as usize {
        count += digits[i].pow(p as u32);
        p += 1 as i32;
    }
    let res: f64 = count as f64 / n as f64;
    if res % 1.0 == 0.0 {
        res as i64
    } else {
        -1
    }

    // OR YOU CAN USE THIS CODE FOR BEST PRACTICE
    // let r: i64 = n
    //     .to_string()
    //     .chars()
    //     .map(|c| (c as i64) - 48)
    //     .enumerate()
    //     .map(|(i, d)| i64::pow(d, p as u32 + i as u32))
    //     .sum();

    // match r % n {
    //     0 => r / n,
    //     _ => -1,
    // }
}

pub fn dotest(n: i64, p: i32, exp: i64) -> () {
    println!(" n: {:?};", n);
    println!("p: {:?};", p);
    let ans = run(n, p);
    println!(" actual:\n{:?};", ans);
    println!("expect:\n{:?};", exp);
    println!(" {};", ans == exp);
    assert_eq!(ans, exp);
    println!("{};", "-");
}

#[test]
pub fn basic_tests() {
    dotest(89, 1, 1);
    dotest(92, 1, -1);
    dotest(46288, 3, 51);
}
