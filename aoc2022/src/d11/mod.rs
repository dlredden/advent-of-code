use std::collections::{HashMap, VecDeque};

pub fn run() -> (String, String) {
    const DATA: &str = include_str!("input.txt");
    (part1(DATA).to_string(), part2(DATA).to_string())
}

struct Monkey {
    inventory: VecDeque<i32>,
    worry_factor: Option<i32>,
    worry: fn(&mut i32, Option<i32>),
    test_factor: i32,
    test: fn(i32, i32) -> bool,
    throw_to: HashMap<bool, usize>,
    inspection_count: i32,
}

impl Monkey {
    fn new(
        inventory: VecDeque<i32>,
        worry_factor: Option<i32>,
        worry: fn(&mut i32, Option<i32>),
        test_factor: i32,
        throw_to: HashMap<bool, usize>,
    ) -> Monkey {
        Monkey {
            inventory,
            worry_factor,
            worry,
            test_factor,
            test: test_worry,
            throw_to,
            inspection_count: 0,
        }
    }

    pub fn inventory_len(&self) -> usize {
        self.inventory.len()
    }

    pub fn inspect(&mut self, x: &mut i32) -> bool {
        self.inspection_count += 1;
        (self.worry)(x, self.worry_factor);
        *x /= 3; // relief
        (self.test)(*x, self.test_factor)
    }

    pub fn add_to_inventory(&mut self, x: i32) {
        self.inventory.push_back(x);
    }

    pub fn get_from_inventory(&mut self) -> i32 {
        self.inventory.pop_front().unwrap()
    }
}

fn worry_multiply(x: &mut i32, multiplier: Option<i32>) {
    if let Some(value) = multiplier {
        *x *= value;
    } else {
        *x = x.pow(2)
    }
}

fn worry_add(x: &mut i32, adder: Option<i32>) {
    *x += adder.unwrap()
}

fn test_worry(x: i32, divisor: i32) -> bool {
    x % divisor == 0
}

fn parse_input(lines: &[&str]) -> Vec<Monkey> {
    let mut monkeys: Vec<Monkey> = Vec::new();
    let filtered: Vec<&&str> = lines.iter().filter(|x| !x.is_empty()).collect();
    let monkey = filtered.chunks_exact(6);

    for line in monkey {
        let inventory = line[1].split(':').collect::<Vec<&str>>()[1]
            .split(',')
            .collect::<Vec<&str>>()
            .iter()
            .map(|x| x.trim().parse::<i32>().unwrap())
            .collect::<VecDeque<i32>>();

        let op = line[2].split_whitespace().collect::<Vec<&str>>();
        let worry = match op[4] {
            "*" => worry_multiply,
            "+" => worry_add,
            _ => panic!("Invalid worry"),
        };
        let worry_factor: Option<i32> = if op[5] == "old" {
            None
        } else {
            Some(op[5].parse::<i32>().unwrap())
        };
        let test_factor = line[3].split_whitespace().collect::<Vec<&str>>()[3]
            .trim()
            .parse::<i32>()
            .unwrap();
        let mut throw_to: HashMap<bool, usize> = HashMap::new();
        throw_to.insert(
            true,
            line[4].split_whitespace().collect::<Vec<&str>>()[5]
                .parse::<usize>()
                .unwrap(),
        );
        throw_to.insert(
            false,
            line[5].split_whitespace().collect::<Vec<&str>>()[5]
                .parse::<usize>()
                .unwrap(),
        );

        monkeys.push(Monkey::new(
            inventory,
            worry_factor,
            worry,
            test_factor,
            throw_to,
        ));
    }

    monkeys
}

fn part1(data: &str) -> i32 {
    let lines: Vec<&str> = data.lines().collect();
    let mut monkeys = parse_input(&lines);
    let num_monkeys = monkeys.len();

    for _i in 0..20 {
        for m in 0..num_monkeys {
            println!("i: {}, m: {}", _i, m);
            while monkeys[m].inventory_len() > 0 {
                let mut x = monkeys[m].get_from_inventory();

                let result = monkeys[m].inspect(&mut x);
                let throw_to = monkeys[m].throw_to[&result];
                monkeys[throw_to].add_to_inventory(x);
            }
        }
    }

    monkeys.sort_by(|a, b| b.inspection_count.cmp(&a.inspection_count));

    monkeys[0].inspection_count * monkeys[1].inspection_count
}

fn part2(data: &str) -> i32 {
    let lines: Vec<&str> = data.lines().collect();
    let _monkeys = parse_input(&lines);
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
