use regex::Regex;
use std::collections::HashMap;

pub fn run() -> (String, String) {
    const DATA: &str = include_str!("input.txt");
    (part1(DATA).to_string(), part2(DATA).to_string())
}

fn part1(data: &str) -> i32 {
    let lines: Vec<&str> = data.lines().collect();
    let mut directories: HashMap<String, i32> = HashMap::new();
    let mut current_dir: String = "root".to_string();
    let re = Regex::new(r"/[a-zA-Z]*$").unwrap();

    for line in lines {
        let command = line.split_whitespace().collect::<Vec<&str>>();
        if command[0] == "$" {
            if command[1] == "ls" {
                continue;
            }
            if command[1] == "cd" {
                if command[2] == "/" {
                    directories.insert(current_dir.to_string(), 0);
                } else if command[2] == ".." {
                    current_dir = re.replace(&current_dir, "").to_string();
                } else {
                    current_dir = current_dir + "/" + command[2];
                }
            }
        } else if command[0] == "dir" {
            directories.insert(current_dir.to_string() + "/" + command[1], 0);
        } else {
            let size = command[0].parse::<i32>().unwrap();
            let dir = directories.get_mut(&current_dir).unwrap();
            *dir += size;
            let mut tmp_dir = current_dir.to_string();
            while tmp_dir != "root" {
                tmp_dir = re.replace(&tmp_dir, "").to_string();
                let tmp_dir = directories.get_mut(&tmp_dir).unwrap();
                *tmp_dir += size;
            }
        }
    }

    let sum = directories
        .iter()
        .filter(|(_d, s)| s < &&100000)
        .map(|(_d, s)| s)
        .sum();

    sum
}

fn part2(data: &str) -> i32 {
    let lines: Vec<&str> = data.lines().collect();
    let mut directories: HashMap<String, i32> = HashMap::new();
    let mut current_dir: String = "root".to_string();
    let re = Regex::new(r"/[a-zA-Z]*$").unwrap();

    for line in lines {
        let command = line.split_whitespace().collect::<Vec<&str>>();
        if command[0] == "$" {
            if command[1] == "ls" {
                continue;
            }
            if command[1] == "cd" {
                if command[2] == "/" {
                    directories.insert(current_dir.to_string(), 0);
                } else if command[2] == ".." {
                    current_dir = re.replace(&current_dir, "").to_string();
                } else {
                    current_dir = current_dir + "/" + command[2];
                }
            }
        } else if command[0] == "dir" {
            directories.insert(current_dir.to_string() + "/" + command[1], 0);
        } else {
            let size = command[0].parse::<i32>().unwrap();
            let dir = directories.get_mut(&current_dir).unwrap();
            *dir += size;
            let mut tmp_dir = current_dir.to_string();
            while tmp_dir != "root" {
                tmp_dir = re.replace(&tmp_dir, "").to_string();
                let tmp_dir = directories.get_mut(&tmp_dir).unwrap();
                *tmp_dir += size;
            }
        }
    }

    const DRIVE_SIZE: i32 = 70000000;
    const REQUIRED_FREE: i32 = 30000000;

    let free_space = REQUIRED_FREE - (DRIVE_SIZE - directories.get(&"root".to_string()).unwrap());

    // println!("Free Space: {} ", free_space);

    let delete_dir_size = directories
        .iter()
        .filter(|(_d, s)| s >= &&free_space)
        .map(|(_d, s)| s)
        .min()
        .unwrap();

    *delete_dir_size
}

#[cfg(test)]
mod test {
    use super::*;
    const INPUT: &str = include_str!("input.test.txt");

    #[test]
    fn p1() {
        assert_eq!(part1(INPUT), 95437);
    }

    #[test]
    fn p2() {
        assert_eq!(part2(INPUT), 24933642);
    }
}
