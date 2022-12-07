use itertools::Itertools;
mod cratenator;

pub fn run() {
    const DATA: &str = include_str!("input.txt");

    println!("D5P1 - Crates on top of stacks: {}", part1(DATA));
    println!("D5P2 - Crates on top of stacks: {}", part2(DATA));
}

fn part1(data: &str) -> String {
    let mut stacks = cratenator::get_stacks(data);
    let moves = cratenator::get_moves(data);

    for m in moves {
        cratenator::move_crates_one_at_a_time(m, &mut stacks);
    }

    let mut top_crates = "".to_string();
    for key in stacks.keys().sorted() {
        if !stacks.get(key).unwrap().is_empty() {
            let x = stacks
                .get(key)
                .unwrap()
                .front()
                .unwrap()
                .chars()
                .nth(1)
                .unwrap();
            top_crates.push(x);
        }
    }

    top_crates
}

fn part2(data: &str) -> String {
    let mut stacks = cratenator::get_stacks(data);
    let moves = cratenator::get_moves(data);

    for m in moves {
        cratenator::move_crates_multiple_at_a_time(m, &mut stacks);
    }

    let mut top_crates = "".to_string();
    for key in stacks.keys().sorted() {
        if !stacks.get(key).unwrap().is_empty() {
            let x = stacks
                .get(key)
                .unwrap()
                .front()
                .unwrap()
                .chars()
                .nth(1)
                .unwrap();
            top_crates.push(x);
        }
    }

    top_crates
}

#[cfg(test)]
mod test {
    use super::*;
    const INPUT: &str = include_str!("input.test.txt");

    #[test]
    fn p1() {
        assert_eq!(part1(INPUT), "CMZ");
    }

    #[test]
    fn p2() {
        assert_eq!(part2(INPUT), "MCD");
    }
}
