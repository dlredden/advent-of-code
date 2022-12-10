use std::cmp::Ordering;

pub fn run() -> (String, String) {
    const DATA: &str = include_str!("input.txt");
    (part1(DATA).to_string(), part2(DATA).to_string())
}

fn parse_input(lines: &Vec<&str>) -> Vec<Vec<i32>> {
    let mut trees: Vec<Vec<i32>> = Vec::new();

    for line in lines {
        trees.push(
            line.chars()
                .map(|c| c.to_digit(10).unwrap() as i32)
                .collect(),
        );
    }
    trees
}

fn is_visible(trees: &Vec<Vec<i32>>, x: usize, y: usize) -> bool {
    // Check if we are on the edge
    if x == 0 || y == 0 || x == trees[0].len() - 1 || y == trees.len() - 1 {
        return true;
    }

    let mut visible_up = true;
    let mut visible_down = true;
    let mut visible_left = true;
    let mut visible_right = true;

    // Look up for taller trees
    for i in 0..y {
        if trees[i][x] >= trees[y][x] {
            visible_up = false;
            break;
        }
    }

    // Look down for taller trees
    for i in y + 1..trees.len() {
        if trees[i][x] >= trees[y][x] {
            visible_down = false;
            break;
        }
    }

    // Look left for taller trees
    for i in 0..x {
        if trees[y][i] >= trees[y][x] {
            visible_left = false;
            break;
        }
    }

    // Look right for taller trees
    for i in x + 1..trees[0].len() {
        if trees[y][i] >= trees[y][x] {
            visible_right = false;
            break;
        }
    }

    visible_up || visible_down || visible_left || visible_right
}

fn part1(data: &str) -> i32 {
    let lines: Vec<&str> = data.lines().collect();
    let trees = parse_input(&lines);
    let mut visible_trees: i32 = 0;

    for y in 0..trees.len() {
        for x in 0..trees[0].len() {
            if is_visible(&trees, x, y) {
                visible_trees += 1;
            }
        }
    }

    visible_trees
}

fn scenic_score(trees: &Vec<Vec<i32>>, x: usize, y: usize) -> i32 {
    // Look up for taller trees
    let mut score_up = 0;
    for i in 0..y {
        match trees[i][x].cmp(&trees[y][x]) {
            Ordering::Less => score_up += 1,
            Ordering::Equal => {
                score_up += 1;
                break;
            }
            Ordering::Greater => {
                score_up += 1;
                break;
            }
        }
    }

    // Look down for taller trees
    let mut score_down = 0;
    for i in y + 1..trees.len() {
        match trees[i][x].cmp(&trees[y][x]) {
            Ordering::Less => score_down += 1,
            Ordering::Equal => {
                score_down += 1;
                break;
            }
            Ordering::Greater => {
                score_down += 1;
                break;
            }
        }
    }

    // Look left for taller trees
    let mut score_left = 0;
    for i in 0..x {
        if trees[y][i] <= trees[y][x] {
            score_left += 1;
            if trees[y][i] == trees[y][x] {
                break;
            }
        } else {
            break;
        }
    }

    // Look right for taller trees
    let mut score_right = 0;
    for i in x + 1..trees[0].len() {
        if trees[y][i] <= trees[y][x] {
            score_right += 1;
            if trees[y][i] == trees[y][x] {
                break;
            }
        } else {
            break;
        }
    }

    // println!(
    //     "({},{}) {} {} {} {} {}",
    //     x,
    //     y,
    //     score_up,
    //     score_down,
    //     score_left,
    //     score_right,
    //     score_up * score_down * score_left * score_right
    // );
    score_up * score_down * score_left * score_right
}

fn part2(data: &str) -> i32 {
    let lines: Vec<&str> = data.lines().collect();
    let trees = parse_input(&lines);
    let mut max_score = 0;
    for y in 0..trees.len() {
        for x in 0..trees[0].len() {
            let score = scenic_score(&trees, x, y);
            // println!("{} {} {}", x, y, score);
            if score > max_score {
                max_score = score;
            }
        }
    }
    max_score
}

#[cfg(test)]
mod test {
    use super::*;
    const INPUT: &str = include_str!("input.test.txt");

    #[test]
    fn p1() {
        assert_eq!(part1(INPUT), 21);
    }

    #[test]
    fn p2() {
        assert_eq!(part2(INPUT), 8);
    }
}
