pub fn run() {
	let data = include_str!("input.txt");

	println!("Sum of items in both rucksacks: {}", part1(&data));

	// let p2_results = part2(&data);
	// println!("My Rochambeau score with strategy: {}", p2_results);
}

fn part1(data: &str) -> isize {
	let rucksack: Vec<&str> = data.lines().collect();
	
	157
}

#[cfg(test)]
mod test {
	use super::*;
	const INPUT: &str = include_str!("input.test.txt");

	#[test]
	fn p1() {
		assert_eq!(part1(INPUT), 157);
	}

	// #[test]
	// fn d2p2() {
	// 	assert_eq!(part2(INPUT), 12);
	// }
}