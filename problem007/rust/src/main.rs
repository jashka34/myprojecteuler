use rust::prime::is_prime;
fn main() {
    println!("Problem 007");
    const MAX: u64 = 10000;
    let mut i:u64 = 1;
    let mut prime_count: u64 = 0;
    while prime_count < MAX {
        i += 2;
        if is_prime(i) {
            prime_count += 1;
        }
    }

    println!("i={}, prime_count={}", i, prime_count);
}
