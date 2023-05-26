pub mod count_bits;
pub mod playing_with_digits;
pub mod unique_in_order;

fn main() {
    println!("count_bits! {}", count_bits::run(4));
    println!(
        "unique_in_order! {:?}",
        unique_in_order::run("AAAABBBCCDAABBB".chars())
    );
    println!("playing_with_digits! {:?}", playing_with_digits::run(89, 1));
}
