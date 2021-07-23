fn prn_cur(fib_prev1: i32, fib_prev2: i32, fib: i32) {
    //println!("{} {} {}", fib_prev1, fib_prev2, fib);
}

fn main() {
    let mut fib: i32 = 1;
    let mut fib_prev1: i32 = 0;
    let mut fib_prev2: i32 = 0;
    let mut sum: i32 = 0;
    while fib < 4_000_000 {
        prn_cur(fib_prev1, fib_prev2, fib);
        fib_prev2 = fib_prev1;
        fib_prev1 = fib;
        fib = fib_prev1 + fib_prev2; 
        //println!("    -> {}", fib);
        prn_cur(fib_prev1, fib_prev2, fib);
        if fib % 2 == 0 {
            sum += fib;
        }
    }
    println!("sum: {}", sum);
}
