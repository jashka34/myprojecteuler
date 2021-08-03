use rust::prime::{get_vec_prime_from_sieve_of_eratosthenes};
fn main() {
    println!("Problem 010");
    let prime_vec = get_vec_prime_from_sieve_of_eratosthenes(2_000_000);
    // println!("{:?}", prime_vec);
    let mut sum: u64 = 0;
    for i in 2..prime_vec.len() {
        sum += prime_vec[i];
    }
    println!("sum={}", sum);
}
