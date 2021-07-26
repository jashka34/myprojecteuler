pub fn is_palindrome(s: &String) -> bool {
    // let mut ret: bool = false;
    let len2: usize;
    if s.len() % 2 == 0 {
        len2 = s.len()/2;
    } else {
        len2 = s.len()/2+1;
    }
    // println!("s={} len={} len/2={} len2={}", s, s.len(), (s.len()/2), len2);
    let s1 = &s[..(s.len()/2)];
    let s2 = &s[len2..];
    let s2rev:String = s2.chars().rev().collect();
    // println!("{} {} {}", s1, s2, s2rev);
    s1 == s2rev
}
//}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn is_palindrome_work() {
    
        assert!( is_palindrome(&String::from("11")));
        assert!( is_palindrome(&String::from("111")));
        assert!(!is_palindrome(&String::from("12")));
        assert!(!is_palindrome(&String::from("123")));
        assert!(!is_palindrome(&String::from("1222")));
        assert!( is_palindrome(&String::from("1221")));
        assert!( is_palindrome(&String::from("12321")));
        assert!(!is_palindrome(&String::from("123456")));
        assert!(!is_palindrome(&String::from("1234567")));
        assert!( is_palindrome(&String::from("1234321")));
        assert!( is_palindrome(&String::from("12344321")));
     } 
 
 }
