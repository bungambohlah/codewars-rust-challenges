pub fn run(m: u64, n: u64) -> Vec<(u64, u64)> {
    let mut res = Vec::new();

    // m must less than n
    if m > n {
        return res;
    }

    for i in m..=n {
        let sum = get_divisors(i).iter().fold(0.0, |acc, &x| acc + x * x);
        let square_root = sum.sqrt();
        if square_root % 1.0 == 0.0 {
            res.push((i, sum as u64));
        }
    }
    res
}

pub fn get_divisors(n: u64) -> Vec<f64> {
    let mut divisors = Vec::new();

    for i in 1..=n / 2 {
        if n % i == 0 {
            divisors.push(i as f64);
        }
    }

    // convert from u64 to Vec<u64>
    divisors.push(n as f64);
    divisors
}

pub fn testing(m: u64, n: u64, exp: Vec<(u64, u64)>) -> () {
    assert_eq!(run(m, n), exp)
}

#[test]
fn basics_list_squared() {
    testing(1, 250, vec![(1, 1), (42, 2500), (246, 84100)]);
    testing(1, 250, vec![(1, 1), (42, 2500), (246, 84100)]);
    testing(42, 250, vec![(42, 2500), (246, 84100)]);
    testing(300, 600, vec![]);
    testing(600, 1500, vec![(728, 722500), (1434, 2856100)]);
    testing(1500, 1800, vec![(1673, 2856100)]);
    testing(1800, 2000, vec![(1880, 4884100)]);
    testing(2000, 2200, vec![]);
    testing(2200, 5000, vec![(4264, 24304900)]);
    testing(
        5000,
        10000,
        vec![(6237, 45024100), (9799, 96079204), (9855, 113635600)],
    );
}
