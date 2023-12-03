pub fn run() -> (String, String) {
    const DATA: &str = include_str!("input.txt");
    (part1(DATA).to_string(), part2(DATA).to_string())
}

fn part1(data: &str) -> i32 {
    let lines: Vec<&str> = data.lines().collect();

    for line in lines {
        println!("{}", line);
    }

		0
}

fn part2(data: &str) -> i32 {
    let lines: Vec<&str> = data.lines().collect();

    for line in lines {
        println!("{}", line);
    }

		0
}

#[cfg(test)]
mod test {
    use super::*;
    const INPUT1: &str = include_str!("input.test.txt");

    #[test]
    fn p1() {
        assert_eq!(part1(INPUT1), 0);
    }

    #[test]
    fn p2() {
        assert_eq!(part2(INPUT2), 0);
    }
}
