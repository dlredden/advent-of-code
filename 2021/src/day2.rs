use std::fs;

struct Position {
	horizontal: i32,
	depth: i32,
	aim: i32,
}

impl Position {
	pub fn up(&mut self, value: i32) {
		self.aim -= value
	}
	
	pub fn down(&mut self, value: i32) {
		self.aim += value
	}
	
	pub fn forward(&mut self, value: i32) {
		self.horizontal += value;
		self.depth += self.aim * value;
	}

	pub fn product(&self) -> i32 {
		self.depth * self.horizontal
	}
}

pub fn run() {
	let contents = fs::read_to_string("./data/day2.txt")
		.expect("Something went wrong reading the file");

	let lines: Vec<&str> = contents
		.split("\n").collect();
    
	let mut position = Position {
		horizontal: 0,
		depth: 0,
		aim: 0,
	};

	for line in lines {
		if line.is_empty() { continue }
		let pos: Vec<&str> = line.split(" ").collect();
		match pos[0] {
			"forward" => position.forward(pos[1].parse::<i32>().unwrap()),
			"up" => position.up(pos[1].parse::<i32>().unwrap()),
			"down" => position.down(pos[1].parse::<i32>().unwrap()),
			_ => panic!("Not a valid direction")
		}
	}

	println!("Position product: {:?}", position.product());
}
