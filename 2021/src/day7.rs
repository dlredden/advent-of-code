use std::fs;
use std::cmp;
use std::collections::HashMap;

pub fn run() {
	let data = fs::read_to_string("./data/day7.txt").expect("Error reading file!");
	
	let fuel_costs_p1 = part1(&data);
	let mut min_fuel_cost: usize = *fuel_costs_p1.get(&1).unwrap();
	for (k, v) in fuel_costs_p1.iter() {
		if *v < min_fuel_cost {
			min_fuel_cost = *v
		}
	}

	println!("Least amount of crab fuel part 1: {}", min_fuel_cost);

	let fuel_costs_p2 = part2(&data);
	let mut min_fuel_cost: usize = *fuel_costs_p2.get(&1).unwrap();
	for (k, v) in fuel_costs_p2.iter() {
		if *v < min_fuel_cost {
			min_fuel_cost = *v
		}
	}

	println!("Least amount of crab fuel part 2: {}", min_fuel_cost);
}

fn part1(data: &str) -> HashMap<usize, usize> {
	let mut results: HashMap<usize, usize> = HashMap::new();
	let positions: Vec<usize> = data.split(",").map(|s| s.parse().unwrap()).collect();
	let crabs_at_position: HashMap<&usize, usize> = positions.iter().fold(HashMap::new(), |mut f, v| {
        *f.entry(v).or_insert(0) += 1;
        f
    });
	let max_pos = positions.iter().max().unwrap();
	let min_pos = positions.iter().min().unwrap();

	for pos in *min_pos..*max_pos {
		for (crab_pos, crabs) in crabs_at_position.iter() {
			let fuel_per_crab = cmp::max(*crab_pos, &pos) - cmp::min(*crab_pos, &pos);
			*results.entry(pos).or_insert(0) += fuel_per_crab * crabs;
		}
	}

	return results;
}

fn part2(data: &str) -> HashMap<usize, usize> {
	let mut results: HashMap<usize, usize> = HashMap::new();
	let positions: Vec<usize> = data.split(",").map(|s| s.parse().unwrap()).collect();
	let crabs_at_position: HashMap<&usize, usize> = positions.iter().fold(HashMap::new(), |mut f, v| {
        *f.entry(v).or_insert(0) += 1;
        f
    });
	let max_pos = positions.iter().max().unwrap();
	let min_pos = positions.iter().min().unwrap();

	for pos in *min_pos..*max_pos {
		for (crab_pos, crabs) in crabs_at_position.iter() {
			let positions_to_move = cmp::max(*crab_pos, &pos) - cmp::min(*crab_pos, &pos);
			let fuel_per_crab = (positions_to_move * (positions_to_move + 1)) / 2;
			*results.entry(pos).or_insert(0) += fuel_per_crab * crabs;
		}
	}

	return results;
}

#[cfg(test)]
mod test {
	use super::*;

	const INPUT: &str = "16,1,2,0,4,2,7,1,2,14";

	#[test]
	fn p1() {
		assert_eq!(*part1(INPUT).get(&1).unwrap(), 41 as usize);
		assert_eq!(*part1(INPUT).get(&2).unwrap(), 37 as usize);
		assert_eq!(*part1(INPUT).get(&3).unwrap(), 39 as usize);
		assert_eq!(*part1(INPUT).get(&10).unwrap(), 71 as usize);
	}

	#[test]
	fn p2() {
		assert_eq!(*part2(INPUT).get(&5).unwrap(), 168 as usize);
	}
}