use std::io;

pub mod filediver;

pub fn run(dir: &str, show: bool, size: u64){
    filediver::scan(&dir, show, size);
}