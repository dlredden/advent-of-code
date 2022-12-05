use std::borrow::Cow;

use itertools::Itertools;

mod cratenator;

pub fn run() {
    const DATA: &str = include_str!("input.txt");

    println!("D5P1 - Crates on top of stacks: {}", part1(&DATA));
    // println!("D5P2 - All overlapping pairs: {}", part2(&DATA));
}

fn part1(data: &str) -> String {
    let mut stacks = cratenator::get_stacks(data);
    // println!("Stacks: {:?}", stacks);
    let moves = cratenator::get_moves(data);
    // println!("Moves: {:?}", moves);

    for m in moves {
        cratenator::move_crates(m, &mut stacks);
    }

    // println!("Stacks: {:?}", stacks);

    let mut top_crates = "".to_string();
    for key in stacks.keys().sorted() {
        if stacks.get(key).unwrap().len() > 0 {
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

    println!("Top crates: {}", top_crates);

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

    // #[test]
    // fn p2() {
    //     assert_eq!(part2(INPUT), 4);
    // }
}
