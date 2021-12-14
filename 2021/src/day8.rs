use std::fs;
use regex::Regex;
use itertools::Itertools;
use std::collections::HashMap;
use std::collections::HashSet;

pub fn run() {
	let data = fs::read_to_string("./data/day8.txt").expect("Error reading file!");

	let p1_results = part1(&data);
	println!("Number of unique digits in output values: {}", p1_results);

	let p2_results = part2(&data);
	println!("Sum of all output values: {}", p2_results);
}

fn part1(data: &str) -> usize {
	let mut count: usize = 0;
	let lines: Vec<&str> = data
		.lines()
		.collect();
	
	for line in lines {
		let x: Vec<&str> = line.split(" | ").collect();
		for value in x[1].split(" ") {
			if value.len() == 7 || value.len() == 4 || value.len() == 2 || value.len() == 3 {
				count += 1;
			}
		}
	}
	let input: Vec<&str> = data.split(" | ").collect();

	count
}

fn part2(data: &str) -> usize {
	let mut sum: usize = 0;
	let lines: Vec<&str> = data.lines().collect();
	for line in lines {
		sum += decode_output(line)
	}
	sum
}

fn decode_output(data: &str) -> usize {
	let regex = Regex::new(r"(\s\|\s)|(\s)").unwrap();
	let mut patterns: HashMap<String, usize> = regex
		.split(data)
		.map(|s| s.chars().sorted().collect::<String>())
		.map(|v| (v, 0 as usize))
    .collect::<HashMap<String, usize>>();

	// Set all the easy patterns
	for (k, v) in patterns.iter_mut().filter(|(x, y)| [2,3,4,7].contains(&x.len())) {
		match k.len() {
			2 => *v = 1,
			3 => *v = 7,
			4 => *v = 4,
			7 => *v = 8,
			_ => println!("Unexpected length of signal!")
		}
	}

	let patterns_clone = patterns.clone();
	let mut one_chars: HashSet<&str> = HashSet::new();
	let mut four_chars: HashSet<&str> = HashSet::new();
	
	for (k, v) in patterns_clone.iter() {
		if v == &1 { one_chars = k.split_terminator("").skip(1).collect::<HashSet<&str>>() }
		if v == &4 { four_chars = k.split_terminator("").skip(1).collect::<HashSet<&str>>() }
	}

	// Process 6 character patterns
	for (k, v) in patterns.iter_mut().filter(|(x, y)| x.len() == 6) {
		let chars: HashSet<&str> = k.split("").collect();

		// A 6 character pattern with one char from the 1 pattern is 6
		let mut intersection = chars.intersection(&one_chars);
		if intersection.count() == 1 {
			*v = 6;
			continue
		}

		// A 6 character pattern with all chars from the 4 pattern is 9
		intersection = chars.intersection(&four_chars);
		if intersection.count() == 4 {
			*v = 9;
			continue
		}

		// What's left is 0
		*v = 0
	}

	let patterns_clone_six = patterns.clone();
	let mut six_chars: HashSet<&str> = HashSet::new();
	for (k, v) in patterns_clone_six.iter() {
		if v == &6 { six_chars = k.split_terminator("").skip(1).collect::<HashSet<&str>>() }
	}

	// Process 5 character patterns
	for (k, v) in patterns.iter_mut().filter(|(x, y)| x.len() == 5) {
		let chars: HashSet<&str> = k.split("").collect();

		// A 5 character pattern with all chars in common with 1 is 3
		let mut intersection = chars.intersection(&one_chars);
		if intersection.count() == 2 {
			*v = 3;
			continue
		}

		// A 5 character pattern with 5 chars in common with 6 is 5
		intersection = chars.intersection(&six_chars);
		if intersection.count() == 5 {
			*v = 5;
			continue
		}

		// A 5 character pattern with four chars in common with 6 is 2
		intersection = chars.intersection(&six_chars);
		if intersection.count() == 4 {
			*v = 2;
			continue
		}
	}	
	
	println!("Patterns: {:?}", patterns);

	let output: Vec<String> = data.split(" | ").collect::<Vec<&str>>()[1].split(" ").map(|s| s.chars().sorted().collect::<String>()).collect::<Vec<String>>();
	let mut answer = "".to_string();
	// format!("{:?}{:?}{:?}{:?}", patterns.get(&output[0]).unwrap(), patterns.get(&output[1]).unwrap(), patterns.get(&output[2]).unwrap(), patterns.get(&output[3]).unwrap());;
	for i in output.iter() {
		answer = format!("{}{}", answer, patterns.get(i).unwrap());
	}
	println!("Output {:?}: ", output);
	// println!("Answers {:?}{:?}{:?}{:?}", patterns.get(&output[0]).unwrap(), patterns.get(&output[1]).unwrap(), patterns.get(&output[1]).unwrap(), patterns.get(&output[1]).unwrap());
	println!("Answer {}", answer.parse::<usize>().unwrap());
	
	answer.parse::<usize>().unwrap()
}

#[cfg(test)]
mod test {
	use super::*;

	const INPUT: &str = "be cfbegad cbdgef fgaecd cgeb fdcge agebfd fecdb fabcd edb | fdgacbe cefdb cefbgd gcbe
edbfga begcd cbg gc gcadebf fbgde acbgfd abcde gfcbed gfec | fcgedb cgb dgebacf gc
fgaebd cg bdaec gdafb agbcfd gdcbef bgcad gfac gcb cdgabef | cg cg fdcagb cbg
fbegcd cbd adcefb dageb afcb bc aefdc ecdab fgdeca fcdbega | efabcd cedba gadfec cb
aecbfdg fbg gf bafeg dbefa fcge gcbea fcaegb dgceab fcbdga | gecf egdcabf bgf bfgea
fgeab ca afcebg bdacfeg cfaedg gcfdb baec bfadeg bafgc acf | gebdcfa ecba ca fadegcb
dbcfg fgd bdegcaf fgec aegbdf ecdfab fbedc dacgb gdcebf gf | cefg dcbef fcge gbcadfe
bdfegc cbegaf gecbf dfcage bdacg ed bedf ced adcbefg gebcd | ed bcgafe cdgba cbgef
egadfb cdbfeg cegd fecab cgb gbdefca cg fgcdab egfdb bfceg | gbdfcae bgc cg cgb
gcafb gcf dcaebfg ecagb gf abcdeg gaef cafbge fdbac fegbdc | fgae cfgab fg bagce";

	#[test]
	fn p1() {
		assert_eq!(part1(INPUT), 26);
	}

	// #[test]
	// fn p2() {
	// 	assert_eq!(part1(INPUT), 26);
	// }
}