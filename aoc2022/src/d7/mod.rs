mod file_system;
use file_system::Dir;

pub fn run() -> (String, String) {
    const DATA: &str = include_str!("input.txt");
    (part1(DATA).to_string(), part2(DATA).to_string())
}

fn part1(data: &str) -> i32 {
    let lines: Vec<&str> = data.lines().collect();
    let mut dir: Dir = Dir::new("/".to_string(), None);
    let mut current_dir = &mut dir;

    for line in lines {
        let command = line.split_whitespace().collect::<Vec<&str>>();
        // println!("Command {:?}", command);
        if command[0] == "$" {
            if command[1] == "cd" {
                // current_dir.ls();
                current_dir = dir.change_dir(command[2].to_string());
            }
        } else if command[0] == "dir" {
            // println!("Added dir {} to {}", command[1], current_dir.name);
            current_dir.add_dir(command[1].to_string());
        } else {
            let size = command[0].parse::<i32>().unwrap();
            current_dir.add_file(command[1].to_string(), size);
            // println!("Added file {} with size {}", command[1], size)
        }
    }
    let sizes = dir.get_sizes();
    println!("Sizes: {:?}", sizes);

    let ret: i32 = sizes.iter().filter(|&x| *x < 1000000).sum();
    dir.ls();
    ret
}

fn part2(data: &str) -> i32 {
    0
}

#[cfg(test)]
mod test {
    use super::*;
    const INPUT: &str = include_str!("input.test.txt");

    #[test]
    fn p1() {
        assert_eq!(part1(INPUT), 95437);
    }

    // #[test]
    // fn p2() {
    //     assert_eq!(part2(INPUT), 0);
    // }
}
