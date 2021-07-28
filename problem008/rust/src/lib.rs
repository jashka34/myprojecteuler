use std::fs::File;
use std::io::Read;
use std::env;

pub fn read_file(file_name: &String) -> std::io::Result<String> {
    let mut file = File::open(file_name)?;
    let mut data = String::new();
    file.read_to_string(&mut data)?;
    Ok(data)
}

pub fn print_cur_dir() {
    let path = env::current_dir();
    match path {
        Ok(res) => println!("cur path: {}", res.display()),
        Err(e) => println!("path err: {:?}", e)
    }
}

pub fn gsm(s: &str, len: usize) -> u64 {
    let mut max: u64 = 1;
    let mut maxs: &str = "";
    // let mut last_l_sum: i32 = 0;
    for i in 0..(s.len()-len+1) {
        // print!("{}) ", i);
        let st: &str = &s[i..(i+len)];
        // println!("{}", st);
        let mut max3: u64 = 1;
        for ch in st.chars() {
            // print!("    {} ch: {} ", i, ch);
            // println!("chn: {:?}", ch.to_digit(10));
            match ch.to_digit(10) {
                Some(chn) => max3 *= chn as u64,
                None => println!("{} is not number!", ch)
            }
        }
        // println!("    max3={}", max3);
        if max3 > max {
            max = max3;
            maxs = st;
        }
    }
    println!("max={} (from string: {})", max, maxs);
    max 
}


#[cfg(test)]
mod tests {
    
    use super::*;

    #[test]
    fn gsm_work() {
        let r1 = gsm(&"12345", 2);
        let r2 = gsm(&"123456789", 3);
        let r3 = gsm(&"123456789475", 3);
        let r4 = gsm(&"1234567894759999999998888999999888888999999999", 13);
        assert_eq!(r1, 20);
        assert_eq!(r2, 504);
        assert_eq!(r3, 504);
        assert_eq!(r4, 1586874322944);
    }
 
    #[test]
    fn read_file_work() {
        // print_cur_dir();
        match read_file(&"test.txt".to_string()) {
            Ok(res) => { println!("Ok!! {:?}", res); 
                         assert_eq!(res, "123\n");
                       },
            Err(e) =>  { println!("Ne Ok... {:?}", e); assert!(1==2);}
        }

    }

}
