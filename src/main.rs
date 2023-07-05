pub mod count_bits;
pub mod does_my_number_look_big;
pub mod exes_and_ohs;
pub mod int32_to_ipv4;
pub mod isbn10_validation;
pub mod moving_zeros_to_the_end;
pub mod playing_with_digits;
pub mod primes_in_numbers;
pub mod rot_13;
pub mod unique_in_order;

fn main() {
    println!("count_bits! {}", count_bits::run(4));
    println!(
        "unique_in_order! {:?}",
        unique_in_order::run("AAAABBBCCDAABBB".chars())
    );
    println!("playing_with_digits! {:?}", playing_with_digits::run(89, 1));
    println!("exes and ohs! {:?}", exes_and_ohs::run("xo"));
    println!(
        "moving zeros to the end! {:?}",
        moving_zeros_to_the_end::run(&[1, 2, 0, 1, 0, 1, 0, 3, 0, 1])
    );
    println!(
        "Does my number look big in this?! {:?}",
        does_my_number_look_big::run(7)
    );
    println!("ROT13 {:?}", rot_13::run("test"));
    println!(
        "ISBN-10 Validation {:?}",
        isbn10_validation::run("1112223339")
    );
    println!("int32 to IPv4 {:?}", int32_to_ipv4::run(2154959208));
    println!("Primes in numbers {:?}", primes_in_numbers::run(7775460));
}
