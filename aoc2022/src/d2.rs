use std::fs;

pub fn run() {
	let data = fs::read_to_string("./data/d2.txt").expect("Error reading file!");

	let p1_results = part1(&data);
	println!("My Rochambeau total score: {}", p1_results);

	// let p2_results = part2(&data);
	// println!("Calories carried by top 3 Elves: {}", p2_results);
}

fn part1(data: &str) -> isize {
	let rounds: Vec<&str> = data.lines().collect();
	let mut scores = [0,0];

	for round in rounds {
		let plays: Vec<&str> = round.split_whitespace().collect();
		// println!("{}.{}.", plays[0], plays[1]);
		let round_score = calculate_scores(plays[0], plays[1]);
		scores[0] += round_score[0];
		scores[1] += round_score[1];
	}
	// println!("{:?}", scores);
	scores[1]
}

fn calculate_scores(theirs: &str, mine: &str) -> [isize; 2] {
	let their_play: isize = match theirs {
			"A" => 1,
			"B" => 2,
			"C" => 3,
			_ => 0
	};
	let my_play: isize = match mine {
			"X" => 1,
			"Y" => 2,
			"Z" => 3,
			_ => 0
	};
	
	let diff: isize = (their_play - my_play).try_into().unwrap();
	// println!("{}.{}.{}", their_play, my_play, diff);
	
	match diff {
		1 | -2 => [their_play + 6, my_play],
		-1 | 2 => [their_play, my_play + 6],
		_ => [their_play + 3, my_play + 3]
	}
}

#[cfg(test)]
mod test {
	use super::*;

	const INPUT: &str = "A Y
B X
C Z
";

	#[test]
	fn d2p1() {
		assert_eq!(part1(INPUT), 15);
	}

	// #[test]
	// fn d2p2() {
	// 	assert_eq!(part2(INPUT), 45000);
	// }
}