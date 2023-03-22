use std::io;

pub mod filediver;

pub fn run(){

    let stdin = io::stdin();

    println!("What directory would you like to scan?");
    let mut dir_input = String::new();
    stdin.read_line(&mut dir_input);

    filediver::scan(&dir_input, false, 10000);
}