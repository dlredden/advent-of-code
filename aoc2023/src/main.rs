use std::{
    collections::HashMap,
    thread::{self, JoinHandle},
};

mod d1;

fn main() {
    let mut handles: HashMap<i32, JoinHandle<(String, String)>> = HashMap::new();
    let mut results: HashMap<i32, (String, String)> = HashMap::new();

    // Spawn threads for each day
    for i in 1..2 {
        let handle = thread::spawn(move || match i {
            1 => d1::run(),
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
