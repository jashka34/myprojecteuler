
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

pub fn is_vec_prime(v: &Vec<u64>) -> bool {
    // println!("{:?}", v);
    for val in v {
        // println!("  {}", val);
        if !is_prime(*val) {
            return false;
        }
    }
    return true;
}

pub fn get_vec_prime_from_sieve_of_eratosthenes(n: usize) -> Vec<u64> {

    let mut vec: Vec<u64> = Vec::with_capacity(n);
    
    vec.push(0);
    for i in 1..n {
        vec.push(i as u64);
    }
    // println!("len vec: {}", vec.len());
    let mut p: u64;
    for i in 2..vec.len() {
        p = i as u64;
        // println!("{}) p={} ", i, p);
        if p * p > n as u64 {
            break;
        }
        let mut cur: usize = (p as usize) * (p as usize);
        while cur < n {
            vec[cur] = 0;
            cur = cur + p as usize;
            // println!("    cur={}", cur);
        }
    }
    // println!("{:?}", vec);
    let mut vec2: Vec<u64> = Vec::with_capacity(n/3);
    for i in 2..n {
        if vec[i] != 0 {
            vec2.push(vec[i]);
        }
    }

    return vec2;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn is_vec_prime_work() {
        let mut v: Vec<u64> = vec! [2, 3, 5, 7];
        assert!(is_vec_prime(&v));
        v.push(4);
        assert!(!is_vec_prime(&v));
    }
    
    #[test]
    fn eratosthenes_work() {
        let evec = get_vec_prime_from_sieve_of_eratosthenes(20);
        println!("{:?}", evec);
        let chvec: Vec<u64> = vec![2, 3, 5, 7, 11, 13, 17, 19];
        assert_eq!(evec, chvec);
    }
    
    #[test]
    fn is_prime_works() {
        assert!(is_prime(2));
        assert!(!is_prime(4));
        assert!(is_prime(5));
        assert!(!is_prime(15));
        assert!(is_prime(29));
        assert!(is_prime(73));
        assert!(!is_prime(74));
        assert!(!is_prime(91));
        assert!(is_prime(97));
        assert!(!is_prime(98));
        assert!(!is_prime(129));
        assert!(is_prime(367));
        assert!(is_prime(643));
        assert!(!is_prime(651));
        assert!(is_prime(1009));
        assert!(is_prime(129707));
        assert!(is_prime(259321));
        assert!(!is_prime(259324));
    }
}
