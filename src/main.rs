pub mod count_bits;
pub mod exes_and_ohs;
pub mod moving_zeros_to_the_end;
pub mod playing_with_digits;
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
}
