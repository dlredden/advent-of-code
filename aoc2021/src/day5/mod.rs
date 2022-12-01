use std::fs;
use regex::Regex;
use itertools::Itertools;
use std::collections::HashMap;

mod segment;
use segment::Bresenham;

pub fn run() {
	let data = fs::read_to_string("./data/day5.txt").expect("Error reading file!");
	let lines: Vec<&str> = data.lines().collect();
	let pattern = Regex::new(r"\s+->\s+").unwrap();
	let mut points: HashMap<(isize, isize), usize> = HashMap::new();
	
	for line in lines.iter() {
		let data: Vec<(isize, isize)> = pattern.split(line)
			.map(|s| s
				.split(",")
				.map(|x| x.parse().unwrap()).collect_tuple::<(isize, isize)>().unwrap())
			.collect();
		
		// let (x1, y1) = data[0];
		// let (x2, y2) = data[1];
		// if x1 == x2 || y1 == y2 {
			// println!("{:?} -> {:?}", data[0], data[1]);
			for (x, y) in Bresenham::new(data[0], data[1]) {
				let counter = points.entry((x, y)).or_insert(0);
				*counter += 1;
        // println!("{}, {}", x, y);
    	}
		// }
	}
	println!("Points with at least 2 overlaps: {:?}", points.values().filter(|&&v| v >= 2).collect::<Vec<&usize>>().len());
}