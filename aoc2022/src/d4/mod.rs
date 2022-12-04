use itertools::Itertools;
pub fn run() {
    let data = include_str!("input.txt");

    println!("D4P1 - Fully contained pairs: {}", part1(&data));
    println!("D4P2 - All overlapping pairs: {}", part2(&data));
}

fn part1(data: &str) -> usize {
    let pairs: Vec<&str> = data.lines().collect();
    let mut fully_contained = 0;

    for pair in pairs {
        let x = pair.split(",").collect_vec();

        let (x1, y1) = x[0]
            .split("-")
            .map(|s| s.parse::<u32>().unwrap())
            .collect_tuple()
            .unwrap();

        let (x2, y2) = x[1]
            .split("-")
            .map(|s| s.parse::<u32>().unwrap())
            .collect_tuple()
            .unwrap();

        if (x1 <= x2 && y1 >= y2) || (x1 >= x2 && y1 <= y2) {
            fully_contained += 1;
        }
    }

    fully_contained
}

fn part2(data: &str) -> usize {
    let pairs: Vec<&str> = data.lines().collect();
    let mut fully_contained = 0;

    for pair in pairs {
        let x = pair.split(",").collect_vec();

        let (x1, y1) = x[0]
            .split("-")
            .map(|s| s.parse::<u32>().unwrap())
            .collect_tuple()
            .unwrap();

        let (x2, y2) = x[1]
            .split("-")
            .map(|s| s.parse::<u32>().unwrap())
            .collect_tuple()
            .unwrap();

        if (x1 <= x2 && y1 >= y2)
            || (x1 >= x2 && y1 <= y2)
            || (x1 <= x2 && y1 <= y2 && y1 >= x2)
            || (x1 >= x2 && y1 >= y2 && y2 >= x1)
        {
            fully_contained += 1;
        }
    }

    fully_contained
}

#[cfg(test)]
mod test {
    use super::*;
    const INPUT: &str = include_str!("input.test.txt");

    #[test]
    fn p1() {
        assert_eq!(part1(INPUT), 2);
    }

    #[test]
    fn p2() {
        assert_eq!(part2(INPUT), 4);
    }
}
