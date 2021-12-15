use std::fs;

pub fn run() {
	let data = fs::read_to_string("./data/day9.txt").expect("Error reading file!");

	let p1_results = part1(&data);
	println!("Low point risk level: {}", p1_results);

	let p2_results = part2(&data);
	println!("Product of three largest basins: {}", p2_results);
}

fn part1(data: &str) -> usize {
	let lines: Vec<&str> = data
		.lines()
		.collect();

	let mut data_vec: Vec<Vec<usize>> = Vec::new();
	let columns = lines[0].len();
	let rows = lines.len();

	for line in lines {
		let v: Vec<usize> = line.split_terminator("").skip(1).map(|x| x.parse::<usize>().unwrap()).collect::<Vec<usize>>();
		data_vec.push(v);
	}

	println!("{} {} {:?}", data_vec.len(), data_vec[0].len(), data_vec);
	
	let mut low_point_sum: usize = 0;
	for r in 0..rows {
		for c in 0..columns {
			if is_low_point(&data_vec, (r, c)) {
				println!("LP {} {:?}", data_vec[r][c], (r, c));
				low_point_sum += data_vec[r][c] + 1
			}
		}
	}
	low_point_sum
}

fn part2(data: &str) -> usize {
	let lines: Vec<&str> = data
		.lines()
		.collect();
	0
}

fn is_low_point(data: &Vec<Vec<usize>>, (r, c): (usize, usize)) -> bool {
	let value = data[r][c];
	let columns = data[0].len();
	let rows = data.len();

	// Look up
	if r > 0 { // Can I look up
		if data[r - 1][c] <= value {
			return false
		}
	}

	// Look down
	if r < rows - 1 { // Can I look down
		if data[r + 1][c] <= value {
			return false
		}
	}

	// Look left
	if c > 0 { // Can I look left
		if data[r][c - 1] <= value {
			return false
		}
	}

	// Look right
	if c < columns - 1 { // Can I look right
		if data[r][c + 1] <= value {
			return false
		}
	}

	true
}

#[cfg(test)]
mod test {
	use super::*;

	const INPUT: &str = "2199943210
3987894921
9856789892
8767896789
9899965678";

	#[test]
	fn p1() {
		assert_eq!(part1(INPUT), 15);
	}

	#[test]
	fn p2() {
		assert_eq!(part1(INPUT), 1134);
	}
}