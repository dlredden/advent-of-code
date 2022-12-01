use std::fs;
use regex::Regex;
use futures::future;

mod board;
use board::Board;
use board::Number;

pub async fn run() {
	let data = fs::read_to_string("./data/day4.txt").expect("Error reading file!");
	let mut lines: Vec<&str> = data.lines().collect();
	
	// The first row are the drawn numbers
	let bingo_numbers: Vec<usize> = lines
		.remove(0)
		.split(",")
		.map(|s| s.parse().unwrap())
		.collect();

	let pattern = Regex::new(r"\s+").unwrap();
	let mut boards: Vec<Board> = Vec::new();
	let mut board_number = 1;

	while lines.len() > 0 {
		lines.remove(0); // Remove one empty line
		let mut board = Board::new(board_number);
		for _i in 0..5 { // Get 5 lines representing one board
			let line = lines.remove(0).trim_start();
			board.rows.push(pattern
				.split(line)
				.map(|s| Number::new(s.parse().unwrap()))
				.collect());
		}
		boards.push(board);
		board_number += 1;
	}

	for num in bingo_numbers {
		let marks = boards.iter().map(|b| b.mark(num));
		let responses = future::join_all(marks).await;
		let winner: Vec<&isize> = responses.iter().filter(|&&b| b.is_positive()).collect();
		// println!("Responses: {:?} {:?}", responses, winner);
		if winner.len() == 1 {
			println!("Winning Bingo: {}", winner[0]);
			break
		}
	}
}