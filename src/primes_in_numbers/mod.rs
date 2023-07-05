pub fn run(n: i64) -> String {
    let mut result = String::new();
    let mut i = 2;
    let mut num = n;
    while i * i <= num {
        let mut count = 0;
        while num % i == 0 {
            count += 1;
            num /= i;
        }
        if count > 0 {
            result.push_str(&format!(
                "({}{})",
                i,
                if count > 1 {
                    format!("**{}", count)
                } else {
                    "".to_string()
                }
            ));
        }
        i += 1;
    }
    if num > 1 {
        result.push_str(&format!("({})", num));
    }
    result
}

pub fn testing(n: i64, exp: &str) -> () {
    assert_eq!(&run(n), exp)
}

#[test]
fn basics_prime_factors() {
    testing(7775460, "(2**2)(3**3)(5)(7)(11**2)(17)");
    testing(17 * 17 * 93 * 677, "(3)(17**2)(31)(677)");
}
