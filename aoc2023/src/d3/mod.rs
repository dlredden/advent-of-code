pub fn run() -> (String, String) {
    const DATA: &str = include_str!("input.txt");
    (part1(DATA).to_string(), part2(DATA).to_string())
}

fn is_adjacent(grid: &Vec<Vec<char>>, x: usize, y: usize) -> bool {
    let columns = grid[0].len();
    let rows = grid.len();

    // Look up if I can
    if y > 0 {
        // Look straight up
        let up: char = grid[y - 1][x];
        if up != '.' && !up.is_ascii_digit() {
            return true;
        }
        // Look up-left if I can
        if x > 0 {
            let up_left = grid[y - 1][x - 1];
            if up_left != '.' && !up_left.is_ascii_digit() {
                return true;
            }
        }
        // Look up-right if I can
        if x < columns - 1 {
            let up_right = grid[y - 1][x + 1];
            if up_right != '.' && !up_right.is_ascii_digit() {
                return true;
            }
        }
    }

    // Look down if I can
    if y < rows - 1 {
        // Look straight down
        let down = grid[y + 1][x];
        if down != '.' && !down.is_ascii_digit() {
            return true;
        }
        // Look down-left if I can
        if x > 0 {
            let down_left = grid[y + 1][x - 1];
            if down_left != '.' && !down_left.is_ascii_digit() {
                return true;
            }
        }
        // Look down-right if I can
        if x < columns - 1 {
            let down_right = grid[y + 1][x + 1];
            if down_right != '.' && !down_right.is_ascii_digit() {
                return true;
            }
        }
    }

    // Look left if I can
    if x > 0 {
        let left = grid[y][x - 1];
        if left != '.' && !left.is_ascii_digit() {
            return true;
        }
    }

    // Look right if I can
    if x < columns - 1 {
        let right = grid[y][x + 1];
        if right != '.' && !right.is_ascii_digit() {
            return true;
        }
    }

    false
}

fn part1(data: &str) -> i32 {
    let lines: Vec<&str> = data.lines().collect();
    let grid: Vec<Vec<char>> = lines.iter().map(|line| line.chars().collect()).collect();
    let mut part_numbers: Vec<i32> = Vec::new();

    for (y, row) in grid.iter().enumerate() {
        let mut number: String = String::new();
        let mut is_part_number: bool = false;

        for (x, c) in row.iter().enumerate() {
            // If we've reached a digit
            if c.is_ascii_digit() {
                // Add it to the number string
                number.push(*c);
                // If we haven't already set is_part_number to true, check if it's adjacent
                if !is_part_number {
                    is_part_number = is_adjacent(&grid, x, y)
                }
            }

            // If we've reached a non-digit or we've reached the end of the row
            // and we have a number
            if (!c.is_ascii_digit() || x == row.len() - 1) && !number.is_empty() {
                // Add it to the part_numbers array if it's a part number
                if is_part_number || (c.is_ascii_digit() && is_adjacent(&grid, x, y)) {
                    part_numbers.push(number.parse::<i32>().unwrap())
                }
                // Reset number and is_part_number
                number = String::new();
                is_part_number = false;
            }
        }
    }

    part_numbers.iter().sum()
}

fn find_adjoining_numbers(grid: &Vec<Vec<char>>, x: usize, y: usize) -> Vec<i32> {
    let columns = grid[0].len();
    let rows = grid.len();
    let mut adjoining_numbers: Vec<i32> = Vec::new();

    // Look up if I can
    if y > 0 {
        // Look straight up
        let up: char = grid[y - 1][x];
        if up.is_ascii_digit() {
            let mut num: String = char::to_string(&up);
            // Get numbers to the left until we hit a non-digit or the edge
            let mut x_left = x;
            while x_left > 0 {
                x_left -= 1;
                let left = grid[y - 1][x_left];
                if left.is_ascii_digit() {
                    num.insert(0, left);
                } else {
                    break;
                }
            }
            // Get numbers to the right until we hit a non-digit or the edge
            let mut x_right = x;
            while x_right < columns - 1 {
                x_right += 1;
                let right = grid[y - 1][x_right];
                if right.is_ascii_digit() {
                    num.push(right);
                } else {
                    break;
                }
            }
            adjoining_numbers.push(num.parse::<i32>().unwrap());
        } else {
            // Look up-left if I can
            if x > 0 {
                let up_left = grid[y - 1][x - 1];
                if up_left.is_ascii_digit() {
                    let mut num: String = char::to_string(&up_left);
                    // Get numbers to the left until we hit a non-digit or the edge
                    let mut x_left = x - 1;
                    while x_left > 0 {
                        x_left -= 1;
                        let left = grid[y - 1][x_left];
                        if left.is_ascii_digit() {
                            num.insert(0, left);
                        } else {
                            break;
                        }
                    }
                    adjoining_numbers.push(num.parse::<i32>().unwrap());
                }
            }

            // Look up-right if I can
            if x < columns - 1 {
                let up_right = grid[y - 1][x + 1];
                if up_right.is_ascii_digit() {
                    let mut num: String = char::to_string(&up_right);
                    // Get numbers to the right until we hit a non-digit or the edge
                    let mut x_right = x + 1;
                    while x_right < columns - 1 {
                        x_right += 1;
                        let right = grid[y - 1][x_right];
                        if right.is_ascii_digit() {
                            num.push(right);
                        } else {
                            break;
                        }
                    }
                    adjoining_numbers.push(num.parse::<i32>().unwrap());
                }
            }
        }
    }

    // Look down if I can
    if y < rows - 1 {
        // Look straight down
        let down = grid[y + 1][x];
        if down.is_ascii_digit() {
            let mut num: String = char::to_string(&down);
            // Get numbers to the left until we hit a non-digit or the edge
            let mut x_left = x;
            while x_left > 0 {
                x_left -= 1;
                let left = grid[y + 1][x_left];
                if left.is_ascii_digit() {
                    num.insert(0, left);
                } else {
                    break;
                }
            }
            // Get numbers to the right until we hit a non-digit or the edge
            let mut x_right = x;
            while x_right < columns - 1 {
                x_right += 1;
                let right = grid[y + 1][x_right];
                if right.is_ascii_digit() {
                    num.push(right);
                } else {
                    break;
                }
            }
            adjoining_numbers.push(num.parse::<i32>().unwrap());
        } else {
            // Look down-left if I can
            if x > 0 {
                let down_left = grid[y + 1][x - 1];
                if down_left.is_ascii_digit() {
                    let mut num: String = char::to_string(&down_left);
                    // Get numbers to the left until we hit a non-digit or the edge
                    let mut x_left = x - 1;
                    while x_left > 0 {
                        x_left -= 1;
                        let left = grid[y + 1][x_left];
                        if left.is_ascii_digit() {
                            num.insert(0, left);
                        } else {
                            break;
                        }
                    }
                    adjoining_numbers.push(num.parse::<i32>().unwrap());
                }
            }

            // Look down-right if I can
            if x < columns - 1 {
                let down_right = grid[y + 1][x + 1];
                if down_right.is_ascii_digit() {
                    let mut num: String = char::to_string(&down_right);
                    // Get numbers to the right until we hit a non-digit or the edge
                    let mut x_right = x + 1;
                    while x_right < columns - 1 {
                        x_right += 1;
                        let right = grid[y + 1][x_right];
                        if right.is_ascii_digit() {
                            num.push(right);
                        } else {
                            break;
                        }
                    }
                    adjoining_numbers.push(num.parse::<i32>().unwrap());
                }
            }
        }
    }

    // Look left if I can
    if x > 0 {
        let left = grid[y][x - 1];
        if left.is_ascii_digit() {
            let mut num: String = char::to_string(&left);
            // Get numbers to the left until we hit a non-digit or the edge
            let mut x_left = x - 1;
            while x_left > 0 {
                x_left -= 1;
                let left = grid[y][x_left];
                if left.is_ascii_digit() {
                    num.insert(0, left);
                } else {
                    break;
                }
            }
            adjoining_numbers.push(num.parse::<i32>().unwrap());
        }
    }

    // Look right if I can
    if x < columns - 1 {
        let right = grid[y][x + 1];
        if right.is_ascii_digit() {
            let mut num: String = char::to_string(&right);
            // Get numbers to the right until we hit a non-digit or the edge
            let mut x_right = x + 1;
            while x_right < columns - 1 {
                x_right += 1;
                let right = grid[y][x_right];
                if right.is_ascii_digit() {
                    num.push(right);
                } else {
                    break;
                }
            }
            adjoining_numbers.push(num.parse::<i32>().unwrap());
        }
    }

    adjoining_numbers
}

fn part2(data: &str) -> i32 {
    let lines: Vec<&str> = data.lines().collect();
    let grid: Vec<Vec<char>> = lines.iter().map(|line| line.chars().collect()).collect();
    let mut gear_ratios: Vec<i32> = Vec::new();

    for (y, row) in grid.iter().enumerate() {
        for (x, c) in row.iter().enumerate() {
            // If we've reached a digit
            if c == &'*' {
                let adjoining_numbers: Vec<i32> = find_adjoining_numbers(&grid, x, y);
                // If we have adjoining numbers
                if adjoining_numbers.len() == 2 {
                    gear_ratios.push(adjoining_numbers[0] * adjoining_numbers[1]);
                }
            }
        }
    }

    gear_ratios.iter().sum()
}

#[cfg(test)]
mod test {
    use super::*;
    const INPUT: &str = include_str!("input.test.txt");

    #[test]
    fn p1() {
        assert_eq!(part1(INPUT), 4361);
    }

    #[test]
    fn p2() {
        assert_eq!(part2(INPUT), 467835);
    }
}
