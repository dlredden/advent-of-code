use std::collections::HashMap;

pub fn run() -> (String, String) {
    const DATA: &str = include_str!("input.txt");
    (part1(DATA).to_string(), part2(DATA).to_string())
}

fn parse_input(lines: &Vec<&str>) -> Vec<Vec<i32>> {
    let mut board: Vec<Vec<i32>> = Vec::new();
    let char_map = get_char_map();

    for line in lines {
        board.push(line.chars().map(|c| char_map[&c]).collect());
    }

    board
}

fn get_char_map() -> HashMap<char, i32> {
    let mut map: HashMap<char, i32> = HashMap::new();

    for i in 1..27 {
        map.insert(((i + 96) as u8) as char, i);
    }
    map.insert('S', 0);
    map.insert('E', 27);

    map
}

// function to find the position with value x
fn get_pos(board: &[Vec<i32>], x: i32) -> (usize, usize) {
    for (i, row) in board.iter().enumerate() {
        for (j, col) in row.iter().enumerate() {
            if *col == x {
                return (i, j);
            }
        }
    }

    (0, 0)
}

struct Position {
    pos: (usize, usize),
    steps: usize,
}

// Function that uses Dijkstra's algorithm to find the shortest path to position 27
fn find_shortest_path(board: &[Vec<i32>]) -> usize {
    let mut queue: Vec<Position> = Vec::new();
    let mut visited: Vec<Position> = Vec::new();

    let start_pos = get_pos(board, 0);
    queue.push(Position {
        steps: 0,
        pos: start_pos,
    });

    while !queue.is_empty() {
        let current_pos = queue.remove(0);
        visited.push(current_pos);

        let next_pos = find_nearest_position(
            board,
            current_pos,
            board[current_pos.pos.0][current_pos.pos.1] + 1,
        );

        if next_pos.pos != current_pos.pos {
            queue.push(next_pos);
        }
    }

    visited.last().unwrap().steps
}

fn find_nearest_position(
    board: &[Vec<i32>],
    current_pos: Position,
    value_to_find: i32,
) -> Position {
    // We have found the end
    if value_to_find > 27 {
        return current_pos;
    }

    let mut positions: Vec<Position> = Vec::new();

    // check left for value + 1
    if current_pos.pos.1 > 0
        && (board[current_pos.pos.0][current_pos.pos.1 - 1] == value_to_find
            || board[current_pos.pos.0][current_pos.pos.1 - 1] == value_to_find + 1)
    {
        positions.push(Position {
            steps: 1,
            pos: (current_pos.pos.0, current_pos.pos.1 - 1),
        });
    }

    // check right for value + 1
    if current_pos.pos.1 < board.len() - 1
        && board[current_pos.pos.0][current_pos.pos.1 + 1] == value_to_find
    {
        positions.push(Position {
            steps: 1,
            pos: (current_pos.pos.0, current_pos.pos.1 + 1),
        });
    }

    // check up for value + 1
    if current_pos.pos.0 > 0 && board[current_pos.pos.0 - 1][current_pos.pos.1] == value_to_find {
        positions.push(Position {
            steps: 1,
            pos: (current_pos.pos.0 - 1, current_pos.pos.1),
        });
    }

    // check down for value + 1
    if current_pos.pos.0 < board.len() - 1
        && board[current_pos.pos.0 + 1][current_pos.pos.1] == value_to_find
    {
        positions.push(Position {
            steps: 1,
            pos: (current_pos.pos.0 + 1, current_pos.pos.1),
        });
    }

    // find the positions with the least steps
    let mut next_pos: Position;
    next_pos = positions.pop().unwrap();
    for pos in positions {
        if pos.steps < next_pos.steps {
            next_pos = pos;
        }
    }

    // Add the current_pos steps to the next_position steps
    next_pos.steps += current_pos.steps;

    find_nearest_position(board, next_pos, value_to_find + 1)
}

fn part1(data: &str) -> i32 {
    let lines: Vec<&str> = data.lines().collect();
    let board = parse_input(&lines);
    let starting_pos = get_pos(&board, 0);
    let destination = get_pos(&board, 27);

    let pos = find_nearest_position(
        &board,
        Position {
            pos: starting_pos,
            steps: 0,
        },
        1,
    );

    // print the board
    for row in board {
        for col in row {
            print!("{: >2} ", col);
        }
        println!();
    }
    println!("{:?}", starting_pos);
    println!("{:?}", destination);
    println!("{:?} {}", pos.pos, pos.steps);
    31
}

fn part2(data: &str) -> i32 {
    let _lines: Vec<&str> = data.lines().collect();
    // let _map = parse_input(&lines);

    0
}

#[cfg(test)]
mod test {
    use super::*;
    const INPUT: &str = include_str!("input.test.txt");

    #[test]
    fn p1() {
        assert_eq!(part1(INPUT), 31);
    }

    #[test]
    fn p2() {
        assert_eq!(part2(INPUT), 0);
    }
}
