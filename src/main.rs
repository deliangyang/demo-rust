extern crate regex;

use regex::Regex;

pub mod lib;

fn main() {

    let reg = Regex::new(r"(\d{4})-(\d{2})-(\d{2})").unwrap();
    let caps = reg.captures("2019-04-28").unwrap();
    println!("{}", caps.get(1).unwrap().as_str());
    println!("{}", caps.get(2).unwrap().as_str());
    println!("{}", caps.get(3).unwrap().as_str());
    println!("a + b = {}", lib::add(2, 3));


    let test = Test {
        a: String::from("a")
    };
    test.test();

    println!("the longest is: {}", lib::longest("abc", "a"));
}

trait Default {
    fn test(&self);
}

struct Test {
    a: String
}

impl Default for Test {
    fn test(&self) {
        println!("Hello world {}", self.a);
    }
}


