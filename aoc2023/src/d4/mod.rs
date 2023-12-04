pub fn run() -> (String, String) {
    const DATA: &str = include_str!("input.txt");
    (part1(DATA).to_string(), part2(DATA).to_string())
}

fn part1(data: &str) -> i32 {
    let cards: Vec<&str> = data.lines().collect();
    let mut points = 0;

    for card in cards {
        // Card 1: 41 48 83 86 17 | 83 86  6 31 17  9 48 53
        let split_card = card.split(" | ").collect::<Vec<&str>>();
        let winning_numbers = split_card[0].split(':').collect::<Vec<&str>>()[1]
            .split_whitespace()
            .collect::<Vec<&str>>();
        let my_numbers = split_card[1].split_whitespace().collect::<Vec<&str>>();
        let matches = winning_numbers
            .iter()
            .filter(|&n| my_numbers.contains(n))
            .count();
        match matches {
            0 => continue,
            1 => points += 1,
            m if m > 1 => points += 1 << (m - 1),
            _ => (),
        }
    }

    points
}

fn part2(_data: &str) -> i32 {
    0
}

#[cfg(test)]
mod test {
    use super::*;
    const INPUT: &str = include_str!("input.test.txt");

    #[test]
    fn p1() {
        assert_eq!(part1(INPUT), 13);
    }

    #[test]
    fn p2() {
        assert_eq!(part2(INPUT), 0);
    }
}
