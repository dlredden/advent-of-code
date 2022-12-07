pub fn run() -> (String, String) {
    const DATA: &str = include_str!("input.txt");
    (part1(DATA).to_string(), part2(DATA).to_string())
}

fn part1(data: &str) -> i32 {
    let lines: Vec<&str> = data.lines().collect();
    let mut elves: Vec<i32> = Vec::new();
    let mut elf_count: usize = 0;
    elves.push(0);

    for line in lines {
        if line.is_empty() {
            elf_count += 1;
            elves.push(0);
            continue;
        }

        elves[elf_count] += line.parse::<i32>().unwrap();
    }

    elves.sort();
    elves[elf_count]
}

fn part2(data: &str) -> i32 {
    let lines: Vec<&str> = data.lines().collect();
    let mut elves: Vec<i32> = Vec::new();
    let mut elf_count: usize = 0;
    elves.push(0);

    for line in lines {
        if line.is_empty() {
            elf_count += 1;
            elves.push(0);
            continue;
        }

        elves[elf_count] += line.parse::<i32>().unwrap();
    }

    elves.sort();

    elves[elf_count] + elves[elf_count - 1] + elves[elf_count - 2]
}

#[cfg(test)]
mod test {
    use super::*;
    const INPUT: &str = include_str!("input.test.txt");

    #[test]
    fn p1() {
        assert_eq!(part1(INPUT), 24000);
    }

    #[test]
    fn p2() {
        assert_eq!(part2(INPUT), 45000);
    }
}
