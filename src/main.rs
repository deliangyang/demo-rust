extern crate regex;

use regex::Regex;
use std::{time, thread, env};
use std::time::Duration;

pub mod lib;

extern crate notify;

use notify::{Watcher, RecursiveMode, watcher, DebouncedEvent};
use std::sync::mpsc::channel;

fn watch() {
    let (tx, rx) = channel();

    // Create a watcher object, delivering debounced events.
    // The notification back-end is selected based on the platform.
    let mut watcher = watcher(tx, Duration::from_secs(1)).unwrap();

    // Add a path to be watched. All files and directories at that path and
    // below will be monitored for changes.
    watcher.watch("f://work", RecursiveMode::Recursive).unwrap();

    loop {
        match rx.recv() {
            Ok(event) => println!("{:?}", parse_event(event)),
            Err(e) => println!("watch error: {:?}", e),
        }
    }
}

fn parse_event(event: DebouncedEvent) {
    match event {
        DebouncedEvent::Create(path_buf) => if path_buf.is_file() { println!("this is the filename: {:?}", path_buf) },
        DebouncedEvent::Write(path_buf) => {
            println!("write this file: {:?}", path_buf)
        }
        _ => {
        }
    }
}

macro_rules! test {
    () => {
        println!("test")
    };
}

fn main(){
    test!();

    let args: Vec<String> = env::args().collect();

    for argument in args {
        println!("argument: {}", argument)
    }

    watch();

    let reg = Regex::new(r"(\d{4})-(\d{2})-(\d{2})").unwrap();
    let caps = reg.captures("2019-04-28").unwrap();
    println!("{}", caps.get(1).unwrap().as_str());
    println!("{}", caps.get(2).unwrap().as_str());
    println!("{}", caps.get(3).unwrap().as_str());
    println!("a + b = {}", lib::add(2, 3));

    let thd = thread::spawn(move || {
        thread::sleep(time::Duration::from_millis(1000));
        println!("hello world")
    });
    thd.join().unwrap();

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


