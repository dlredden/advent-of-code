use std::fs;
use rayon::prelude::*;
use std::sync::{Mutex};
use std::collections::HashMap;

pub fn run() {
	let data = fs::read_to_string("./data/day6.txt").expect("Error reading file!");
	
	let p1_results = part1(&data);
	let p2_results = part2(&data);
    println!("Fish after 80 days: {}", p1_results);
    println!("Fish after 256 days: {}", p2_results);
}

fn part1(data: &str) -> usize {
    let mut fish: Vec<u8> = data.split(",").map(|s| s.parse().unwrap()).collect();
    for day in 0..80 {
        finish_day1(&mut fish);
    }
    return fish.len();
}

fn part2(data: &str) -> usize {
    let numbers: Vec<u8> = data.split(",").map(|s| s.parse().unwrap()).collect();
    let mut fish: HashMap<u8, usize> = HashMap::new();

    for n in numbers.iter() {
        let counter = fish.entry(*n).or_insert(0);
	    *counter += 1;
    }

    for day in 0..256 {
        println!("Day {}: {}", day, sum_fish(&fish));
        finish_day2(&mut fish);
    }

    return sum_fish(&fish);
}

fn sum_fish(fish: &HashMap<u8, usize>) -> usize {
    let mut count = 0;
    for (k, v) in fish.iter() {
        count += *v;
    }
    return count;
}

fn finish_day2(fish: &mut HashMap<u8, usize>) {
    let mut tmp: HashMap<u8, usize> = HashMap::new();

    for (k, v) in fish.iter() {
        if *k == 0 {
            let mut counter = tmp.entry(6).or_insert(0);
            *counter += *v;
            counter = tmp.entry(8).or_insert(0);
            *counter = *v;
        } else {
            let counter = tmp.entry(*k - 1).or_insert(0);
	        *counter += *v;
        }
    }

    *fish = tmp;
}

fn finish_day1(fish: &mut Vec<u8>) {
    let new_fish: Mutex<usize> = Mutex::new(0);
    let mut chunks: Vec<Vec<u8>> = fish.chunks(10000000).map(|s| s.into()).collect();

    chunks.par_iter_mut().for_each(|chunk|
       for i in chunk.iter_mut() {
            if *i == 0 {
                *i = 6;
                let mut locked= new_fish.lock().unwrap();
                *locked += 1;
            } else {
                *i -= 1;
            }
        } 
    );

    *fish = vec![0; 0];
    for mut chunk in chunks {
        fish.append(&mut chunk)
    }
    fish.append(&mut vec![8; *new_fish.lock().unwrap()]);
}

#[cfg(test)]
mod test {
    use super::*;

    const INPUT: &str = "3,4,3,1,2";

    #[test]
    fn p1() {
        assert_eq!(part1(INPUT), 5934);
    }

    #[test]
    fn p2() {
        assert_eq!(part2(INPUT), 26984457539); //26984457539
    }
}