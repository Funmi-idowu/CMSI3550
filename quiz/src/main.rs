// main.rs

use std::env::args;

fn main() {
    let argument: Vec<String> = args().collect();
    let address = argument.get(1).cloned().unwrap_or_default();
    let num: Vec<&str> = address.split(":").collect();
    for nums in num {
        println!("{}", nums);
    }
}
