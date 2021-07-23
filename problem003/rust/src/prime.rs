
    pub fn is_prime(n: u64) -> bool {
        //println!("------------------");
        //println!("is_prime: {}", n);
        if n == 2 {
            return true;
        }
        if n % 2 == 0 {
            return false;
        } else {
            let sq = (n as f64).sqrt();
            let maxi: u64 = (sq as u64) + 1;
            //let mut max: i32 = 0;
            let mut flag_is_prime: bool = true;
            for i in 2..maxi {
                //println!("{}", i);
                if n % i == 0 {
                    flag_is_prime = false;
                    break;
                }
            }
            return flag_is_prime;
        }
    }


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn is_prime_works() {
        assert!(is_prime(2));
        assert!(!is_prime(4));
        assert!( is_prime(5));
        assert!(!is_prime(15));
        assert!( is_prime(29));
        assert!( is_prime(73));
        assert!(!is_prime(74));
        assert!(!is_prime(91));
        assert!( is_prime(97));
        assert!(!is_prime(98));
        assert!(!is_prime(129));
        assert!( is_prime(367));
        assert!( is_prime(643));
        assert!(!is_prime(651));
        assert!( is_prime(1009));
        assert!( is_prime(129707));
        assert!( is_prime(259321));
        assert!(!is_prime(259324));
    }
}
