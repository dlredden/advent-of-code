use itertools::Itertools;
use std::collections::VecDeque;

pub fn run() -> (String, String) {
    const DATA: &str = include_str!("input.txt");
    (part1(DATA).to_string(), part2(DATA).to_string())
}

fn part1(data: &str) -> i32 {
    let mut count = 0;
    let mut signal: VecDeque<char> = VecDeque::new();

    let chars = data.lines().next().unwrap().chars();
    for c in chars {
        count += 1;

        if signal.len() < 4 {
            signal.push_back(c);
        } else {
            signal.pop_front();
            signal.push_back(c);
        }

        if signal.iter().unique().count() == 4 {
            return count;
        }
    }

    count
}

fn part2(data: &str) -> i32 {
    let mut count = 0;
    let mut signal: VecDeque<char> = VecDeque::new();

    let chars = data.lines().next().unwrap().chars();
    for c in chars {
        count += 1;
        let length = signal.len();
        if length < 14 {
            signal.push_back(c);
        } else {
            signal.pop_front();
            signal.push_back(c);
        }
        if signal.iter().unique().count() == 14 {
            return count;
        }
    }

    count
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn p1() {
        assert_eq!(part1("mjqjpqmgbljsphdztnvjfqwrcgsmlb"), 7);
        assert_eq!(part1("bvwbjplbgvbhsrlpgdmjqwftvncz"), 5);
        assert_eq!(part1("nppdvjthqldpwncqszvftbrmjlhg"), 6);
        assert_eq!(part1("nznrnfrfntjfmvfwmzdfjlvtqnbhcprsg"), 10);
        assert_eq!(part1("zcfzfwzzqfrljwzlrfnpqdbhtmscgvjw"), 11);
    }

    #[test]
    fn p2() {
        assert_eq!(part2("mjqjpqmgbljsphdztnvjfqwrcgsmlb"), 19);
        assert_eq!(part2("bvwbjplbgvbhsrlpgdmjqwftvncz"), 23);
        assert_eq!(part2("nppdvjthqldpwncqszvftbrmjlhg"), 23);
        assert_eq!(part2("nznrnfrfntjfmvfwmzdfjlvtqnbhcprsg"), 29);
        assert_eq!(part2("zcfzfwzzqfrljwzlrfnpqdbhtmscgvjw"), 26);
    }
}
