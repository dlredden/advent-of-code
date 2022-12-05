use regex::Regex;
use std::collections::HashMap;
use std::collections::VecDeque;

pub fn get_stacks(data: &str) -> HashMap<u32, VecDeque<&str>> {
    let stack_numbers = Regex::new(r"\s\d\s\W").unwrap();
    let stacks = Regex::new(r"\s\s\s\s|\[[A-Z]\]").unwrap();

    let num_stacks = stack_numbers
        .find_iter(data)
        .map(|s| s.as_str().trim().parse::<u32>().unwrap())
        .collect::<Vec<u32>>()
        .pop()
        .unwrap();

    let mut stack_map: HashMap<u32, VecDeque<&str>> = HashMap::new();
    let mut current_stack = 1;
    for stack in stacks.find_iter(data) {
        if current_stack > num_stacks {
            current_stack = 1;
        }

        // Only add stacks that have crates
        if stack.as_str().trim() != "" {
            stack_map
                .entry(current_stack)
                .and_modify(|v| {
                    v.push_back(stack.as_str().trim());
                })
                .or_insert(VecDeque::from([stack.as_str().trim()]));
        }
        current_stack += 1;
    }

    stack_map
}

pub fn get_moves(data: &str) -> Vec<&str> {
    let moves = Regex::new(r"move.*").unwrap();
    moves.find_iter(data).map(|m| m.as_str()).collect()
}

pub fn move_crates_one_at_a_time(instruction: &str, stacks: &mut HashMap<u32, VecDeque<&str>>) {
    let move_regex = Regex::new(r"move\s(\d+)\sfrom\s(\d+)\sto\s(\d+)").unwrap();
    let captures = move_regex.captures(instruction).unwrap();
    let mut num_crates = captures.get(1).unwrap().as_str().parse::<u32>().unwrap();
    let from_stack = captures.get(2).unwrap().as_str().parse::<u32>().unwrap();
    let to_stack = captures.get(3).unwrap().as_str().parse::<u32>().unwrap();

    while num_crates > 0 {
        let crate_to_move = stacks.get_mut(&from_stack).unwrap().pop_front().unwrap();
        stacks.get_mut(&to_stack).unwrap().push_front(crate_to_move);
        num_crates -= 1;
    }
}

pub fn move_crates_multiple_at_a_time(
    instruction: &str,
    stacks: &mut HashMap<u32, VecDeque<&str>>,
) {
    let move_regex = Regex::new(r"move\s(\d+)\sfrom\s(\d+)\sto\s(\d+)").unwrap();
    let captures = move_regex.captures(instruction).unwrap();
    let num_crates = captures.get(1).unwrap().as_str().parse::<u32>().unwrap();
    let from_stack = captures.get(2).unwrap().as_str().parse::<u32>().unwrap();
    let to_stack = captures.get(3).unwrap().as_str().parse::<u32>().unwrap();

    let mut crates_to_move = VecDeque::new();
    for _ in 0..num_crates {
        crates_to_move.push_front(stacks.get_mut(&from_stack).unwrap().pop_front().unwrap());
    }

    for _ in 0..num_crates {
        stacks
            .get_mut(&to_stack)
            .unwrap()
            .push_front(crates_to_move.pop_front().unwrap());
    }
}
