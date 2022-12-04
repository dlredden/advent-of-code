use std::fs;

pub fn run() {
    let data = fs::read_to_string("./data/d1.txt").expect("Error reading file!");

    println!("D1P1 - Calories carried by top Elf: {}", part1(&data));
    println!("D1P2 - Calories carried by top 3 Elves: {}", part2(&data));
}

fn part1(data: &str) -> usize {
    let lines: Vec<&str> = data.lines().collect();
    let mut elves: Vec<usize> = Vec::new();
    let mut elf_count: usize = 0;
    elves.push(0);

    for line in lines {
        if line.len() == 0 {
            elf_count += 1;
            elves.push(0);
            continue;
        }

        elves[elf_count] += line.parse::<usize>().unwrap();
    }

    elves.sort();
    elves[elf_count]
}

fn part2(data: &str) -> usize {
    let lines: Vec<&str> = data.lines().collect();
    let mut elves: Vec<usize> = Vec::new();
    let mut elf_count: usize = 0;
    elves.push(0);

    for line in lines {
        if line.len() == 0 {
            elf_count += 1;
            elves.push(0);
            continue;
        }

        elves[elf_count] += line.parse::<usize>().unwrap();
    }

    elves.sort();

    elves[elf_count] + elves[elf_count - 1] + elves[elf_count - 2]
}

#[cfg(test)]
mod test {
    use super::*;

    const INPUT: &str = "1000
2000
3000

4000

5000
6000

7000
8000
9000

10000
";

    #[test]
    fn d1p1() {
        assert_eq!(part1(INPUT), 24000);
    }

    #[test]
    fn d1p2() {
        assert_eq!(part2(INPUT), 45000);
    }
}
