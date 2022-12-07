use std::{
    collections::HashMap,
    thread::{self, JoinHandle},
};

mod d1;
mod d2;
mod d3;
mod d4;
mod d5;
mod d6;

fn main() {
    let mut handles: HashMap<i32, JoinHandle<(String, String)>> = HashMap::new();
    let mut results: HashMap<i32, (String, String)> = HashMap::new();

    for i in 1..7 {
        let handle = thread::spawn(move || match i {
            1 => d1::run(),
            2 => d2::run(),
            3 => d3::run(),
            4 => d4::run(),
            5 => d5::run(),
            6 => d6::run(),
            _ => panic!("Invalid day"),
        });
        handles.insert(i, handle);
    }

    for (day, handle) in handles {
        let result: (String, String) = handle.join().unwrap();
        results.insert(day, result);
    }

    let mut pairs: Vec<(&i32, &(String, String))> = results.iter().collect();
    pairs.sort_by(|a, b| a.0.cmp(b.0));
    for (day, (p1, p2)) in pairs {
        println!("Day {}.1: {}\nDay {}.2: {}", day, p1, day, p2);
    }
}
