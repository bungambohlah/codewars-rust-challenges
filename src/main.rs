pub mod count_bits;
pub mod unique_in_order;

fn main() {
    println!("count_bits! {}", count_bits::run(4));
    println!(
        "unique_in_order! {:?}",
        unique_in_order::run("AAAABBBCCDAABBB".chars())
    );
}
