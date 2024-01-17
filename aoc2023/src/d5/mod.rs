pub fn run() -> (String, String) {
    const DATA: &str = include_str!("input.txt");
    (part1(DATA).to_string(), part2(DATA).to_string())
}

fn part1(data: &str) -> i32 {
    let lines: Vec<&str> = data.lines().collect();

    for line in lines.iter() {
        if line.is_empty() {
            continue;
        }

        if line.contains(':') {
            println!("{:?}", line.split(':').collect::<Vec<&str>>()[0]);
        }
    }

    35
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
        assert_eq!(part1(INPUT), 35);
    }

    #[test]
    fn p2() {
        assert_eq!(part2(INPUT), 0);
    }
}
