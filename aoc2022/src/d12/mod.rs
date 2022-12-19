use std::collections::HashMap;

pub fn run() -> (String, String) {
    const DATA: &str = include_str!("input.txt");
    (part1(DATA).to_string(), part2(DATA).to_string())
}

fn parse_input(lines: &Vec<&str>) -> Vec<Vec<char>> {
    let mut board: Vec<Vec<char>> = Vec::new();

    for line in lines {
        board.push(line.chars().collect());
    }

    board
}

// function to find the position with value x
fn get_pos(board: &[Vec<char>], x: char) -> Vec<(usize, usize)> {
    let mut pos: Vec<(usize, usize)> = Vec::new();
    for (i, row) in board.iter().enumerate() {
        for (j, col) in row.iter().enumerate() {
            if *col == x {
                pos.push((i, j));
            }
        }
    }

    pos
}

struct Position {
    pos: (usize, usize),
    steps: usize,
}

// Function that uses Dijkstra's algorithm to find the shortest path to position E
fn map_paths(board: &[Vec<char>], starting_pos: Vec<(usize, usize)>) -> Position {
    let mut visited: HashMap<(usize, usize), usize> = HashMap::new();
    let mut queue: Vec<Position> = Vec::new();
    let mut steps = 0;

    for pos in starting_pos {
        queue.push(Position { pos, steps });
    }

    while !queue.is_empty() {
        let pos = queue.remove(0);
        let (row, col) = pos.pos;
        steps = pos.steps;

        // Check if we've reached the end
        if board[row][col] == 'E' {
            return pos;
        }

        // Check if we've already visited this position
        if visited.contains_key(&(row, col)) {
            if visited[&(row, col)] <= steps {
                continue;
            } else {
                visited.remove(&(row, col));
            }
        }

        // Add the position to the visited list
        visited.insert((row, col), steps);

        // Add the adjacent positions to the queue
        if row > 0 && is_valid_move(board, (row, col), (row - 1, col)) {
            queue.push(Position {
                pos: (row - 1, col),
                steps: steps + 1,
            });
        }

        if row < board.len() - 1 && is_valid_move(board, (row, col), (row + 1, col)) {
            queue.push(Position {
                pos: (row + 1, col),
                steps: steps + 1,
            });
        }

        if col > 0 && is_valid_move(board, (row, col), (row, col - 1)) {
            queue.push(Position {
                pos: (row, col - 1),
                steps: steps + 1,
            });
        }

        if col < board[0].len() - 1 && is_valid_move(board, (row, col), (row, col + 1)) {
            queue.push(Position {
                pos: (row, col + 1),
                steps: steps + 1,
            });
        }
    }

    Position { pos: (0, 0), steps }
}

fn get_value(c: char) -> u32 {
    match c {
        'S' => 0,
        'E' => 27,
        _ => c as u32 - 96,
    }
}

fn is_valid_move(board: &[Vec<char>], from_pos: (usize, usize), to_pos: (usize, usize)) -> bool {
    // Make sure move is less than or equal to 1 more than the current position
    if get_value(board[to_pos.0][to_pos.1]) <= get_value(board[from_pos.0][from_pos.1]) + 1 {
        return true;
    }

    false
}

fn part1(data: &str) -> i32 {
    let lines: Vec<&str> = data.lines().collect();
    let board = parse_input(&lines);
    let visited = map_paths(&board, get_pos(&board, 'S'));

    visited.steps as i32
}

fn part2(data: &str) -> i32 {
    let lines: Vec<&str> = data.lines().collect();
    let board = parse_input(&lines);
    let mut starting_pos = get_pos(&board, 'S');
    for pos in get_pos(&board, 'a') {
        starting_pos.push(pos);
    }
    let visited = map_paths(&board, starting_pos);

    // print the board
    // for row in board {
    //     for col in row {
    //         print!("{} ", col);
    //     }
    //     println!();
    // }
    // println!("{:?}", starting_pos);
    // println!("{:?} {}", visited.pos, visited.steps);
    visited.steps as i32
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
        assert_eq!(part2(INPUT), 29);
    }
}
