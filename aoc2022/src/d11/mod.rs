use std::collections::{HashMap, VecDeque};

pub fn run() -> (String, String) {
    const DATA: &str = include_str!("input.txt");
    (part1(DATA).to_string(), part2(DATA).to_string())
}

struct Monkey {
    inventory: VecDeque<i32>,
    worry: fn(&mut i32) -> &mut i32,
    test: fn(i32) -> bool,
    throw_to: HashMap<bool, i32>,
    inspection_count: i32,
}

impl Monkey {
    fn new(
        inventory: VecDeque<i32>,
        worry: fn(&mut i32) -> &mut i32,
        test: fn(i32) -> bool,
        throw_to: HashMap<bool, i32>,
    ) -> Monkey {
        Monkey {
            inventory,
            worry,
            test,
            throw_to,
            inspection_count: 0,
        }
    }

    pub fn inspect(&mut self, x: &mut i32) -> bool {
        self.inspection_count += 1;
        (self.worry)(x);
        *x /= 3;
        (self.test)(*x)
    }

    pub fn add_to_inventory(&mut self, x: i32) {
        self.inventory.push_back(x);
    }

    pub fn get_from_inventory(&mut self) -> i32 {
        self.inventory.pop_front().unwrap()
    }
}

fn parse_input(lines: &Vec<&str>) -> Vec<(String, Option<i32>)> {
    let mut signals: Vec<(String, Option<i32>)> = Vec::new();

    for line in lines {
        let this_signal = line.split_whitespace().collect::<Vec<&str>>();
        let command = this_signal[0].to_string();

        if this_signal.len() == 1 {
            signals.push((command, None));
        } else {
            let value = this_signal[1].parse::<i32>().unwrap();
            signals.push((command, Some(value)));
        }
    }

    signals
}

fn part1(data: &str) -> i32 {
    let lines: Vec<&str> = data.lines().collect();
    let signals = parse_input(&lines);

    10605
}

// function to draw the crt screen
fn draw_crt(cycle: i32, x: i32, crt_output: &mut [Vec<char>]) {
    let row: usize = ((cycle - 1) / 40) as usize;

    let column: usize = if cycle > 40 {
        (cycle - 1) % 40
    } else {
        cycle - 1
    } as usize;

    // println!(
    //     "cycle: {}, x: {}, row: {}, column: {}",
    //     cycle, x, row, column
    // );

    if column == (x - 1) as usize || column == (x + 1) as usize || column == x as usize {
        crt_output[row][column] = '#';
    } else {
        crt_output[row][column] = '.';
    }
}

fn part2(data: &str) -> i32 {
    let lines: Vec<&str> = data.lines().collect();
    let signals = parse_input(&lines);
    0
}

#[cfg(test)]
mod test {
    use super::*;
    const INPUT: &str = include_str!("input.test.txt");

    #[test]
    fn p1() {
        assert_eq!(part1(INPUT), 10605);
    }

    #[test]
    fn p2() {
        assert_eq!(part2(INPUT), 0);
    }
}
