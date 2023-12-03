use std::collections::HashMap;

pub fn run() -> (String, String) {
    const DATA: &str = include_str!("input.txt");
    (part1(DATA).to_string(), part2(DATA).to_string())
}

fn part1(data: &str) -> i32 {
    let games: Vec<&str> = data.lines().collect();
    let mut map: HashMap<&str, i32> = HashMap::new();
    map.insert("red", 12);
    map.insert("green", 13);
    map.insert("blue", 14);

    let mut possible_game_ids: Vec<i32> = Vec::new();

    // line format: Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green
    'game: for game in games {
        let data: Vec<&str> = game.split(':').collect();
        let game_id: i32 = data[0].split_whitespace().collect::<Vec<&str>>()[1]
            .parse::<i32>()
            .unwrap();
        let sets: Vec<&str> = data[1].split(';').collect();

        for set in sets {
            let colors: Vec<&str> = set.split(',').collect();
            for color in colors {
                let c = color.split_whitespace().collect::<Vec<&str>>();
                if map.contains_key(c[1]) && &c[0].parse::<i32>().unwrap() > map.get(c[1]).unwrap()
                {
                    continue 'game;
                }
            }
        }

        possible_game_ids.push(game_id);
    }

    possible_game_ids.iter().sum()
}

fn part2(data: &str) -> i32 {
    let games: Vec<&str> = data.lines().collect();
    let mut game_powers: Vec<i32> = Vec::new();

    for game in games {
        let mut game_cubes: HashMap<&str, i32> = HashMap::new();
        let sets: Vec<&str> = game.split(':').collect::<Vec<&str>>()[1]
            .split(';')
            .collect();

        for set in sets {
            let colors: Vec<&str> = set.split(',').collect();
            for color in colors {
                let c = color.split_whitespace().collect::<Vec<&str>>();
                if game_cubes.contains_key(c[1])
                    && &c[0].parse::<i32>().unwrap() <= game_cubes.get(c[1]).unwrap()
                {
                    // println!("No change; {}:{}", c[1], c[0]);
                    continue;
                }

                let color = game_cubes
                    .entry(c[1])
                    .or_insert(c[0].parse::<i32>().unwrap());
                *color = c[0].parse::<i32>().unwrap();
            }
        }

        game_powers.push(game_cubes.values().product::<i32>());
    }

    game_powers.iter().sum()
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
        assert_eq!(part2(INPUT), 2286);
    }
}
