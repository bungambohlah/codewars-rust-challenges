pub fn run(x: u32) -> String {
    let mut result = vec![1]; // Start with an initial value of 1

    for factor in 2..=x {
        let mut carry = 0;

        for digit in result.iter_mut() {
            let product = *digit * factor + carry;
            *digit = product % 10;
            carry = product / 10;
        }

        while carry > 0 {
            result.push(carry % 10);
            carry /= 10;
        }
    }

    result.iter().rev().map(|digit| digit.to_string()).collect()
}

#[test]
fn sample_tests() {
    assert_eq!(run(1), String::from("1"));
    assert_eq!(run(5), String::from("120"));
    assert_eq!(run(9), String::from("362880"));
    assert_eq!(run(15), String::from("1307674368000"));
    assert_eq!(
            run(100),
            "93326215443944152681699238856266700490715968264381621468592963895217599993229915608941463976156518286253697920827223758251185210916864000000000000000000000000"
        );
}
