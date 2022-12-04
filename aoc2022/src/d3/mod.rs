mod rucksack;

pub fn run() {
    let data = include_str!("input.txt");

    println!("D3P1 - Sum of items in both rucksacks: {}", part1(&data));
    println!("D3P2 - Sum of rucksack badges: {}", part2(&data));
}

fn part1(data: &str) -> usize {
    let rucksacks: Vec<&str> = data.lines().collect();
    let mut priority_sum: usize = 0;

    for rucksack in rucksacks {
        let (half1, half2) = rucksack.split_at(rucksack.len() / 2);
        let common = rucksack::find_common_items(half1, half2);
        // println!("{:?} {:?} {:?}", half1, half2, common);

        for c in common {
            let v = rucksack::PRIORITIES.find(c);
            if !v.is_none() {
                priority_sum += v.unwrap()
            }
        }
    }

    priority_sum
}

fn part2(data: &str) -> usize {
    let mut rucksacks: Vec<&str> = data.lines().collect();
    let mut priority_sum: usize = 0;

    while rucksacks.len() > 0 {
        let mut group: Vec<&str> = Vec::new();
        while group.len() < 3 {
            group.push(rucksacks.pop().unwrap());
        }
        let common = rucksack::find_badges(group);
        for c in common {
            let v = rucksack::PRIORITIES.find(c);
            if !v.is_none() {
                priority_sum += v.unwrap()
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
