use std::fs;

struct Diagnostic {
	sum_bits: Vec<usize>,
	num_entries: usize,
	data: Vec<Vec<char>>
}

impl Diagnostic {
	fn new_from_data(data: Vec<&str>) -> Diagnostic {
		let mut count = 0;
		let mut tmp_sum = vec![0; data[0].len()];
		let mut tmp_data: Vec<Vec<char>> = Vec::with_capacity(data.len() as usize);

		for line in data {
			let digits: Vec<char> = line.chars().collect();
			for (index, value) in digits.iter().enumerate() {
				tmp_sum[index] += value.to_digit(10).unwrap() as usize;
			}

			count += 1;
			tmp_data.push(digits);
		}

		Diagnostic { sum_bits: tmp_sum, num_entries: count, data: tmp_data }
	}

	fn is_one(&self, value: usize) -> bool {
		let half = self.num_entries / 2;
		if value >= half {
			return true
		}
		return false
	}

	fn gamma_rate(&self) -> usize {
		let mut result: String = String::new();
		for value in self.sum_bits.iter() {
			if self.is_one(*value) {
				result.push_str("1")
			} else {
				result.push_str("0")
			}
		}
		return usize::from_str_radix(&result, 2).unwrap();
	}

	fn epsilon_rate(&self) -> usize {
		let mut result: String = String::new();
		for value in self.sum_bits.iter() {
			if self.is_one(*value) {
				result.push_str("0")
			} else {
				result.push_str("1")
			}
		}
		return usize::from_str_radix(&result, 2).unwrap();
	}

	fn power_consumption(&self) {
		println!("Power consumption: {}", self.gamma_rate() * self.epsilon_rate());
	}

	fn reduce_data(&self, data: &Vec<Vec<char>>) -> (Vec<usize>, usize) {
		let mut sums = vec![0; data[0].len()];
		let mut count = 0;

		for line in data {
			for (index, value) in line.iter().enumerate() {
				sums[index] += value.to_digit(10).unwrap() as usize;
			}

			count += 1;
		}

		return (sums, count)
	}

	fn is_one_life_support(&self, data: &Vec<Vec<char>>, index: usize) -> bool {
		let (sums, count) = self.reduce_data(data);
		let half = count as f32 / 2 as f32;

		if sums[index] as f32 >= half {
			return true
		}
		return false
	}

	fn filter_dataset(&self, data: &Vec<Vec<char>>, index: usize, value: char) -> Vec<Vec<char>> {
		let mut results: Vec<Vec<char>> = Vec::new();
		
		for entry in data.iter() {
			if entry[index] == value {
				results.push(entry.to_vec())
			}
		}
		
		return results;
	}

	fn o2_generator_rating(&self, data: &Vec<Vec<char>>, index: usize) -> usize {
		if data.len() == 1 {
			return usize::from_str_radix(&data[0].iter().cloned().collect::<String>(), 2).unwrap();
		}

		if self.is_one_life_support(data, index) {
			self.o2_generator_rating(&self.filter_dataset(data, index, '1'), index + 1)
		} else {
			self.o2_generator_rating(&self.filter_dataset(data, index, '0'), index + 1)
		}
	}

	fn co2_scrubber_rating(&self, data: &Vec<Vec<char>>, index: usize) -> usize {
		if data.len() == 1 {
			return usize::from_str_radix(&data[0].iter().cloned().collect::<String>(), 2).unwrap();
		}

		if self.is_one_life_support(data, index) {
			self.co2_scrubber_rating(&self.filter_dataset(data, index, '0'), index + 1)
		} else {
			self.co2_scrubber_rating(&self.filter_dataset(data, index, '1'), index + 1)
		}
	}

	fn life_support_rating(&self) {
		let o2 = self.o2_generator_rating(&self.data, 0);
		let co2 = self.co2_scrubber_rating(&self.data, 0);
		println!("O2 Generator Rating: {:?}", o2);
		println!("CO2 Scrubber Rating: {:?}", co2);
		println!("Life Support rating: {}", o2 * co2);
	}
}

pub fn run() {
	let data = fs::read_to_string("./data/day3.txt").expect("Error reading file!");
	let lines: Vec<&str> = data
		.lines()
		.collect();

	let diagnostics = Diagnostic::new_from_data(lines);
	diagnostics.power_consumption();
	diagnostics.life_support_rating();
}