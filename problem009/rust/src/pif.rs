const ERR_MSG: &str = "egg1";
#[derive(PartialEq, PartialOrd)]
pub struct Pif {
    a: u32,
    b: u32,
    c: u32,
}

impl Pif {
    pub fn new(a: u32, b: u32, c: u32) -> Pif {
        Pif { a, b, c }
    }
    pub fn check(&self) -> Result<u32, &str> {
        if (self.a >= self.b && self.b >= self.c)
            || self.c * self.c != self.a * self.a + self.b * self.b
        {
            // println!("{}",
            // Err(format!("a={} b={} c={}", self.a, self.b, self.c).as_str())
            Err(ERR_MSG)
        } else {
            Ok(self.c * self.c)
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn pif_create() {
        let p = super::Pif::new(1, 2, 3);
        assert_eq!(p.a, 1);
        assert_eq!(p.b, 2);
        assert_eq!(p.c, 3);
        let p1 = super::Pif::new(4, 5, 6);
        let p2 = super::Pif::new(4, 5, 6);
        assert!(p1 == p2);
    }

    #[test]
    fn pif_check() {
        let p1 = Pif::new(3, 4, 5);
        let ch1 = p1.check();
        println!("check 1: {:?}", ch1);
        assert_eq!(Ok(3 * 3 + 4 * 4), ch1);
        let p1 = Pif::new(200, 375, 425);
        let ch1 = p1.check();
        println!("check 2: {:?}", ch1);
        assert_eq!(Ok(425 * 425), ch1);
    }
}
#[test]
fn pif_check_err() {
    let p1 = Pif::new(1, 2, 3);
    let ch1 = p1.check();
    println!("{:?}", ch1);
    assert_eq!(Err(ERR_MSG), ch1);
}
