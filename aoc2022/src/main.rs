use std::thread;

mod d1;
mod d2;
mod d3;
mod d4;
mod d5;
mod d6;

fn main() {
    let mut handles = vec![];

    for i in 1..6 {
        let handle = thread::spawn(move || match i {
            1 => d1::run(),
            2 => d2::run(),
            3 => d3::run(),
            4 => d4::run(),
            5 => d5::run(),
            6 => d6::run(),
            _ => println!("No such day"),
        });
        handles.push(handle);
    }

    for handle in handles {
        handle.join().unwrap();
    }
}
