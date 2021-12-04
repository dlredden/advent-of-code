use std::fs;

pub fn run() {
	let contents = fs::read_to_string("./data/day1.txt")
		.expect("Something went wrong reading the file");
	
	let mut items: Vec<u32> = Vec::with_capacity(contents.lines().count());
	for line in contents.lines() {
		items.push(line.parse::<u32>().unwrap() as u32)
	}

	let mut sums: Vec<u32> = Vec::with_capacity(items.len() - 2);
	for (index, value) in items.iter().enumerate() {
		if index > items.len() - 3 { continue }
		sums.push(value + items[index + 1] + items[index + 2])
	}

	let mut counter:u32 = 0;
	for (index, value) in items.iter().enumerate() {
		// Skip the last iteration
		if index == items.len() - 1 { continue }

		// If the previous value is lower than the current value
		if value < &items[index + 1] { counter += 1 }
	}

	let mut sum_counter:u32 = 0;
	for (index, value) in sums.iter().enumerate() {
		// Skip the last iteration
		if index == sums.len() - 1 { continue }

		// If the previous value is lower than the current value
		if value < &sums[index + 1] { sum_counter += 1 }
	}

	println!("Number of increases: {:?}", counter);
	println!("Number of increased sums: {:?}", sum_counter);
}
