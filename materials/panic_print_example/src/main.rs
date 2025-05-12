use std::fs::File;
use std::io::Read;

fn main() {
    let greeting_file = File::open("hello.txt").unwrap();
    for byte in greeting_file.bytes() {
        println!("{}", byte.unwrap());
    }
}