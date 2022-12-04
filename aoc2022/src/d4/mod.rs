pub fn run() {
    let data = include_str!("input.txt");

    println!("D4P1 - ???: {}", part1(&data));
    println!("D4P2 - ???: {}", part2(&data));
}

fn part1(data: &str) -> usize {
    12
}

fn part2(data: &str) -> usize {
    12
}

#[cfg(test)]
mod test {
    use super::*;
    const INPUT: &str = include_str!("input.test.txt");

    #[test]
    fn p1() {
        assert_eq!(part1(INPUT), 12);
    }

    #[test]
    fn p2() {
        assert_eq!(part2(INPUT), 12);
    }
}
