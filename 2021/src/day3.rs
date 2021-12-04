use std::fs;

struct Diagnostic {
	sum_bits: Vec<usize>,
	num_entries: usize,
}

impl Diagnostic {
	fn new() -> Diagnostic {
		Diagnostic { sum_bits: vec![0; 12], num_entries: 0 }
	}

	fn process(&mut self, lines: Vec<&str>) {
		for line in lines {
		self.num_entries += 1;
		let digits: Vec<char> = line.chars().collect();
		for (index, value) in digits.iter().enumerate() {
			self.sum_bits[index] += value.to_digit(10).unwrap() as usize;
		}
	}
	}

	fn is_one(&self, value: usize) -> bool {
		let half = self.num_entries / 2;
		if value >= half {
			return true
		}
		return false
	}

	fn gamma_rate(&self) -> String {
		let mut result: String = String::new();
		for value in self.sum_bits.iter() {
			if self.is_one(*value) {
				result.push_str("1")
			} else {
				result.push_str("0")
			}
		}
		return result;
	}

	fn epsilon_rate(&self) -> String {
		let mut result: String = String::new();
		for value in self.sum_bits.iter() {
			if self.is_one(*value) {
				result.push_str("0")
			} else {
				result.push_str("1")
			}
		}
		return result;
	}

	fn power_consumption(&self) {
		let gamma = usize::from_str_radix(&self.gamma_rate(), 2).unwrap();
		let epsilon = usize::from_str_radix(&self.epsilon_rate(), 2).unwrap();
		println!("Gamma: {} {}", self.gamma_rate(), gamma);
		println!("Epsilon: {} {}", self.epsilon_rate(), epsilon);
		println!("Power consumption: {}", gamma * epsilon);
	}
}

pub fn run() {
	let data = fs::read_to_string("./data/day3.txt").expect("Error reading file!");
	let lines: Vec<&str> = data
		.lines()
		.collect();

	let mut diagnostics = Diagnostic::new();
	diagnostics.process(lines);
	diagnostics.power_consumption();
}