use std::fs;

pub fn run() {
	let data = fs::read_to_string("./data/day3.txt").expect("Error reading file!");
}