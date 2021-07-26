use rust::*;
fn main() {
    println!("Pr 04");
    let mut max = 0;
    for n1 in 100..999 {
         for n2 in 100..999 {
             let n = n1 * n2;
             let s: String = n.to_string();
             if is_palindrome(&s) {
                 if n > max {
                     max = n;
                 }
                 // println!("{}", s);
             }
         }
         // println!("{}", n1);
     }
    println!("{}",max);
}
