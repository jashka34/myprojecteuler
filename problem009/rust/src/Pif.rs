#[derive(PartialEq, PartialOrd)]
pub struct Pif {
    a: u32,
    b: u32,
    c: u32,
}

impl Pif {
    fn new(a: u32, b: u32, c: u32) -> Pif {
        Pif { a, b, c }
    }
    fn check(&self) -> Result<u32, &str> {
        if self.a >= self.b || self.c != self.a*self.a + self.b*self.b {
            // println!("{}", 
            Err("a>=b or c2 != a2 + b2")
        } else {
            Ok(self.c)
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
       let p1 = Pif::new(3, 4, 25);
       let ch1 = p1.check();
       println!("{:?}", ch1);
       assert_eq!(Ok(3*3+4*4), ch1);
    }
}
    #[test]
    fn pif_check_err() {
       let p1 = Pif::new(1, 2, 3);
       let ch1 = p1.check();
       println!("{:?}", ch1);
       assert_eq!(Err("a>=b or c2 != a2 + b2"), ch1);
    }
