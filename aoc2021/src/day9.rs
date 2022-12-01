use std::fs;
use std::collections::HashMap;
use std::collections::HashSet;

pub fn run() {
	let data = fs::read_to_string("./data/day9.txt").expect("Error reading file!");

	let p1_results = part1(&data);
	println!("Low point risk level: {}", p1_results);

	let p2_results = part2(&data);
	println!("Product of three largest basins: {}", p2_results);
}

fn part1(data: &str) -> usize {
	let heightmap: Vec<Vec<usize>> = parse_heightmap(&data);
	let low_points: HashMap<(usize, usize), usize> = get_low_points(&heightmap);

	// println!("{} {} {:?}", heightmap.len(), heightmap[0].len(), heightmap);
	
	let mut low_point_sum: usize = 0;
	for (k, v) in low_points.iter() {
		low_point_sum += *v + 1
	}
	low_point_sum
}

fn part2(data: &str) -> usize {
	let heightmap: Vec<Vec<usize>> = parse_heightmap(&data);
	let low_points: HashMap<(usize, usize), usize> = get_low_points(&heightmap);
	let mut basin_size: Vec<usize> = Vec::new();
	let columns = heightmap[0].len();
	let rows = heightmap.len();

	for (k, v) in low_points.iter() {
		basin_size.push(get_basin_size(&heightmap, k));
	}

	basin_size.sort();
	basin_size.reverse();

	basin_size[0] * basin_size[1] * basin_size[2]
}

fn get_basin_size(heightmap: &Vec<Vec<usize>>, point: &(usize, usize)) -> usize {
	let mut filled: HashSet<(usize, usize)> = HashSet::new();
	let rows = heightmap.len() - 1;
	let columns = heightmap[0].len() - 1;
	let mut queue: Vec<(usize, usize)> = Vec::new();

	queue.push(*point);

	while queue.len() > 0 {
		let (row, col) = queue.pop().unwrap();
		
		// skip this node if it's a boundary
		if heightmap[row][col] == 9 {	continue }

		// Count it
		filled.insert((row, col));

		if row > 0 { // up node
			if !filled.contains(&(row - 1, col)) {
				queue.push((row - 1, col))
			}
		}

		if row < rows { // down node
			if !filled.contains(&(row + 1, col)) {
				queue.push((row + 1, col))
			}
		}

		if col > 0 { // left node
			if !filled.contains(&(row, col - 1)) {
				queue.push((row, col - 1))
			}
		}
		
		if col < columns { // right node
			if !filled.contains(&(row, col + 1)) {
				queue.push((row, col + 1))
			}
		}	
	}

	filled.len()
}

fn parse_heightmap(data: &str) -> Vec<Vec<usize>> {
	let lines: Vec<&str> = data
		.lines()
		.collect();

	let mut heightmap: Vec<Vec<usize>> = Vec::new();
	
	for line in lines {
		let v: Vec<usize> = line
			.split_terminator("")
			.skip(1)
			.map(|x| x.parse::<usize>().unwrap())
			.collect::<Vec<usize>>();

		heightmap.push(v);
	}
	heightmap
}

fn get_low_points(data: &Vec<Vec<usize>>) -> HashMap<(usize, usize), usize> {
	let mut low_points: HashMap<(usize, usize), usize> = HashMap::new();
	let columns = data[0].len();
	let rows = data.len();

	for r in 0..rows {
		for c in 0..columns {
			if is_low_point(&data, (r, c)) {
				low_points.insert((r, c), data[r][c]);
			}
		}
	}

	low_points
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
		assert_eq!(part2(INPUT), 1134);
	}
}