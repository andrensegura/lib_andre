extern crate lib_andre;

use lib_andre::{io,os};
//use std::env; //for cmdline args

fn main() {
    //example for prompt()
    let name = io::prompt("Name: ").unwrap();
    
    //example for print_file()
    io::print_file("./test.txt").unwrap();

    //example for is_valid_user()
    //if you want to use cmdline args:
    //  let args: Vec<_> = env::args().collect();
    //  let user = args[1].clone();

    println!("-- Checking user ...");

    if os::is_valid_user(name.as_ref()) {
        println!("-- User is valid: {}", name);
    } else {
        println!("-- User in invalid: {}", name);
    }
}
