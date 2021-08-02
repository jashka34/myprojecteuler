use rust::pif::Pif;
fn main() {
    println!("Problem 009");

    // let p: Pif = Pif::new(3, 4, 26);
    // let ch = p.check();
    // println!("{:?}", ch);

    for a in 1..499 {
        for b in a..500 {
            for c in b..500 {
                let p: Pif = Pif::new(a, b, c);
                let ch = p.check();
                match ch {
                    Ok(res) => {
                        // println!("res={}", res);
                        if a + b + c == 1000 {
                            println!("{} {} {}, res={} a*b*c={}", a, b, c, res, a * b * c);
                        }
                    }
                    Err(_) => {}
                }
            }
        }
    }
}
