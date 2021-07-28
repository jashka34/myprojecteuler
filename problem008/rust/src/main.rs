// use std::io;
use rust::{read_file, gsm};

fn main() {
    println!("Problem 008");
    // let mut file_data: Vec<u8>;
    // print_cur_dir();

    match read_file(&"d.txt".to_string()) {
        Ok(res) =>  { 
                       println!("Ok!! {:?}", res.replace("\n",""));
                       gsm(res.replace("\n","").as_str(), 13);
                    },
        Err(e) => println!("Ne Ok... {:?}", e)
    }

}
