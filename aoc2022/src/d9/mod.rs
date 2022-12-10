use std::{cmp::Ordering, collections::HashMap};

pub fn run() -> (String, String) {
    const DATA: &str = include_str!("input.txt");
    (part1(DATA).to_string(), part2(DATA).to_string())
}

fn parse_input(lines: &Vec<&str>) -> Vec<(String, i32)> {
    let mut moves: Vec<(String, i32)> = Vec::new();

    for line in lines {
        let this_move = line.split_whitespace().collect::<Vec<&str>>();
        let direction = this_move[0].to_string();
        let distance = this_move[1].parse::<i32>().unwrap();
        moves.push((direction, distance));
    }

    moves
}

// function to move the tail within one step of the head
fn move_tail(head: (i32, i32), tail: (i32, i32)) -> (i32, i32) {
    let mut new_tail = tail;
    let distance = (((head.0 - tail.0).pow(2) + (head.1 - tail.1).pow(2)) as f32).sqrt();
    // If the distance is less than 2 then we are close
    if distance < 2.0 {
        return new_tail;
    } else if distance == 2.0 {
        // If the distance is 2 then we can move the tail horizontally or vertically
        // Add one to the tail position furthest from the head
        if head.0 > tail.0 {
            new_tail.0 += 1;
        } else if head.0 < tail.0 {
            new_tail.0 -= 1;
        } else if head.1 > tail.1 {
            new_tail.1 += 1;
        } else if head.1 < tail.1 {
            new_tail.1 -= 1;
        }
    } else {
        // If the distance is greater than 2 then we need to move the tail in both directions
        match head.0.cmp(&tail.0) {
            Ordering::Greater => new_tail.0 += 1,
            Ordering::Less => new_tail.0 -= 1,
            _ => (),
        }
        match head.1.cmp(&tail.1) {
            Ordering::Greater => new_tail.1 += 1,
            Ordering::Less => new_tail.1 -= 1,
            _ => (),
        }
    }

    new_tail
}

fn part1(data: &str) -> i32 {
    let lines: Vec<&str> = data.lines().collect();
    let moves = parse_input(&lines);
    let mut position_history: HashMap<(i32, i32), i32> = HashMap::new();
    let mut head_position: (i32, i32) = (0, 0);
    let mut tail_position: (i32, i32) = (0, 0);

    // Set initial position
    position_history.entry(tail_position).or_insert(1);

    // Iterate through moves
    for (direction, distance) in moves {
        // Move head
        for _i in 0..distance {
            match direction.as_str() {
                "U" => head_position.1 += 1,
                "D" => head_position.1 -= 1,
                "L" => head_position.0 -= 1,
                "R" => head_position.0 += 1,
                _ => panic!("Invalid direction"),
            }

            // Move tail
            tail_position = move_tail(head_position, tail_position);
            let tail_pos = position_history.entry(tail_position).or_insert(0);
            *tail_pos += 1;
            // println!("Head: {:?}, Tail: {:?}", head_position, tail_position)
        }
    }
    position_history.len() as i32
}

fn part2(data: &str) -> i32 {
    let lines: Vec<&str> = data.lines().collect();
    let _moves = parse_input(&lines);
    0
}

#[cfg(test)]
mod test {
    use super::*;
    const INPUT: &str = include_str!("input.test.txt");

    #[test]
    fn p1() {
        assert_eq!(part1(INPUT), 13);
    }

    #[test]
    fn p2() {
        assert_eq!(part2(INPUT), 0);
    }
}
