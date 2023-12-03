use std::collections::HashMap;

pub fn run() -> (String, String) {
    const DATA: &str = include_str!("input.txt");
    (part1(DATA).to_string(), part2(DATA).to_string())
}

fn part1(data: &str) -> i32 {
    let lines: Vec<&str> = data.lines().collect();
		let mut map: HashMap<&str, i32> = HashMap::new();
		map.insert("red", 12);
		map.insert("green", 13);
		map.insert("blue", 14);

		let mut possible_game_ids: Vec<i32> = Vec::new();

		// line format: Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green
    'game: for line in lines {
			let data: Vec<&str> = line.split(':').collect();
			let game_id: i32 = data[0]
				.split_whitespace()
				.collect::<Vec<&str>>()[1].parse::<i32>().unwrap();
			let sets: Vec<&str> = data[1].split(';').collect();

			for set in sets {
				let colors: Vec<&str> = set.split(',').collect();
				for color in colors {
					let c = color.split_whitespace().collect::<Vec<&str>>();
					if map.contains_key(c[1]) && &c[0].parse::<i32>().unwrap() > map.get(c[1]).unwrap() {
						continue 'game;
					}
				}
			}

			possible_game_ids.push(game_id);
    }

		possible_game_ids.iter().sum()
}

fn part2(_data: &str) -> i32 {
    // let lines: Vec<&str> = data.lines().collect();

    // for line in lines {
    //     println!("{}", line);
    // }

		0
}

#[cfg(test)]
mod test {
    use super::*;
    const INPUT: &str = include_str!("input.test.txt");

    #[test]
    fn p1() {
        assert_eq!(part1(INPUT), 8);
    }

    #[test]
    fn p2() {
        assert_eq!(part2(INPUT), 0);
    }
}
