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
mod d7;
mod d8;

fn main() {
    let mut handles: HashMap<i32, JoinHandle<(String, String)>> = HashMap::new();
    let mut results: HashMap<i32, (String, String)> = HashMap::new();

    // Spawn threads for each day
    for i in 1..9 {
        let handle = thread::spawn(move || match i {
            1 => d1::run(),
            2 => d2::run(),
            3 => d3::run(),
            4 => d4::run(),
            5 => d5::run(),
            6 => d6::run(),
            7 => d7::run(),
            8 => d8::run(),
            _ => panic!("Invalid day"),
        });
        handles.insert(i, handle);
    }

    // Join threads and store results
    for (day, handle) in handles {
        let result: (String, String) = handle.join().unwrap();
        results.insert(day, result);
    }

    // Sort results by day
    let mut pairs: Vec<(&i32, &(String, String))> = results.iter().collect();
    pairs.sort_by(|a, b| a.0.cmp(b.0));

    // Print results
    for (day, (p1, p2)) in pairs {
        println!("Day {}.1: {}\nDay {}.2: {}", day, p1, day, p2);
    }
}
