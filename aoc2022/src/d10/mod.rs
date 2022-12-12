pub fn run() -> (String, String) {
    const DATA: &str = include_str!("input.txt");
    (part1(DATA).to_string(), part2(DATA).to_string())
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

// function to test signal strength at the 20th, 60th, 100th, 140th, 180th, and 220th cycle
fn test_signal_strengths(cycle: i32, x: i32, signal_strengths: &mut Vec<i32>) {
    if cycle == 20 || cycle == 60 || cycle == 100 || cycle == 140 || cycle == 180 || cycle == 220 {
        signal_strengths.push(cycle * x);
    }
}

fn part1(data: &str) -> i32 {
    let lines: Vec<&str> = data.lines().collect();
    let signals = parse_input(&lines);
    let mut cycle = 0;
    let mut x = 1;
    let mut signal_strengths: Vec<i32> = Vec::new();

    // loop through signals adding 1 cycle for noops and 2 cycles for addx adding the value to x on the 2nd cycle
    for signal in signals {
        match signal.0.as_str() {
            "noop" => {
                cycle += 1;
                test_signal_strengths(cycle, x, &mut signal_strengths);
            }
            "addx" => {
                cycle += 1;
                test_signal_strengths(cycle, x, &mut signal_strengths);
                cycle += 1;
                test_signal_strengths(cycle, x, &mut signal_strengths);
                x += signal.1.unwrap();
            }
            _ => panic!("Invalid signal"),
        }
    }
    signal_strengths.iter().sum()
}

// function to draw the crt screen
fn draw_crt(cycle: i32, x: i32, crt_output: &mut Vec<Vec<char>>) {
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

fn part2(data: &str) -> String {
    let lines: Vec<&str> = data.lines().collect();
    let signals = parse_input(&lines);
    let mut cycle = 0;
    let mut x = 1;
    let mut crt_output: Vec<Vec<char>> = vec![vec!['.'; 40]; 6];

    // loop through signals adding 1 cycle for noops and 2 cycles for addx adding the value to x on the 2nd cycle
    for signal in signals {
        match signal.0.as_str() {
            "noop" => {
                cycle += 1;
                draw_crt(cycle, x, &mut crt_output);
            }
            "addx" => {
                cycle += 1;
                draw_crt(cycle, x, &mut crt_output);
                cycle += 1;
                draw_crt(cycle, x, &mut crt_output);
                x += signal.1.unwrap();
            }
            _ => panic!("Invalid signal"),
        }
    }

    let ret = crt_output
        .iter()
        .map(|line| line.iter().collect::<String>())
        .collect::<Vec<String>>()
        .join("\n");
    // println!("{} ret", ret);
    "\n".to_string() + ret.as_str()
}

#[cfg(test)]
mod test {
    use super::*;
    const INPUT: &str = include_str!("input.test.txt");

    #[test]
    fn p1() {
        assert_eq!(part1(INPUT), 13140);
    }

    #[test]
    fn p2() {
        assert_eq!(
            part2(INPUT),
            "##..##..##..##..##..##..##..##..##..##..
###...###...###...###...###...###...###.
####....####....####....####....####....
#####.....#####.....#####.....#####.....
######......######......######......####
#######.......#######.......#######....."
        );
    }
}
