use std::io::{stdin, Read};

pub fn get_input(messege: String)-> String{
    println!("{}",messege);
    let mut value = String::new();
    stdin().read_line(&mut value).unwrap();
    value
}