mod rucksack;

pub fn run() -> (String, String) {
    const DATA: &str = include_str!("input.txt");
    (part1(DATA).to_string(), part2(DATA).to_string())
}

fn part1(data: &str) -> i32 {
    let rucksacks: Vec<&str> = data.lines().collect();
    let mut priority_sum: i32 = 0;

    for rucksack in rucksacks {
        let (half1, half2) = rucksack.split_at(rucksack.len() / 2);
        let common = rucksack::find_common_items(half1, half2);
        // println!("{:?} {:?} {:?}", half1, half2, common);

        for c in common {
            let v = rucksack::PRIORITIES.find(c);
            if v.is_some() {
                priority_sum += v.unwrap_or_default() as i32
            }
        }
    }

    priority_sum
}

fn part2(data: &str) -> i32 {
    let mut rucksacks: Vec<&str> = data.lines().collect();
    let mut priority_sum: i32 = 0;

    while !rucksacks.is_empty() {
        let mut group: Vec<&str> = Vec::new();
        while group.len() < 3 {
            group.push(rucksacks.pop().unwrap());
        }
        let common = rucksack::find_badges(group);
        for c in common {
            let v = rucksack::PRIORITIES.find(c);
            if v.is_some() {
                priority_sum += v.unwrap_or_default() as i32
            }
        }
    }

    priority_sum
}

#[cfg(test)]
mod test {
    use super::*;
    const INPUT: &str = include_str!("input.test.txt");

    #[test]
    fn p1() {
        assert_eq!(part1(INPUT), 157);
    }

    #[test]
    fn p2() {
        assert_eq!(part2(INPUT), 70);
    }
}
