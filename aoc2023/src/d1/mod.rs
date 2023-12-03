pub fn run() -> (String, String) {
    const DATA: &str = include_str!("input.txt");
    (part1(DATA).to_string(), part2(DATA).to_string())
}

fn get_number(s: &str) -> i32 {
    if s.parse::<i32>().is_ok() {
        return s.parse::<i32>().unwrap();
    }
    
    match s {
        "one" => 1,
        "two" => 2,
        "three" => 3,
        "four" => 4,
        "five" => 5,
        "six" => 6,
        "seven" => 7,
        "eight" => 8,
        "nine" => 9,
        _ => -1,
    }
}

fn part1(data: &str) -> i32 {
    let lines: Vec<&str> = data.lines().collect();
    let mut calibration_values: Vec<i32> = Vec::new();
    let pattern = regex::Regex::new(r"\d").unwrap();

    for line in lines {
        let matches: Vec<&str> = pattern.find_iter(line).map(|mat| mat.as_str()).collect();
        
        let mut first: i32 = 0;
        let mut last: i32 = 0;

        for m in matches {
            let num = get_number(m);
            if first == 0 {
                first = num;
            }
            last = num;
        }

        let mut s_num = first.to_string().to_owned();
        s_num.push_str(&last.to_string());
        let num = s_num.parse::<i32>().unwrap();
        
        calibration_values.push(num);
    }

    calibration_values.iter().sum()
}

fn part2(data: &str) -> i32 {
    let lines: Vec<&str> = data.lines().collect();
    let mut calibration_values: Vec<i32> = Vec::new();
    let pattern = regex::Regex::new(r"(\d|one|two|three|four|five|six|seven|eight|nine)").unwrap();

    for line in lines {
        let mut first = 0;
        let mut last = 0;

        // loop through line 5 characters at a time
        for i in 0..line.len() {
            let end = if i + 5 > line.len() {
                line.len()
            }
            else { i + 5 };
            
            let s = &line[i..end];
            let matches: Vec<&str> = pattern.find_iter(s).map(|mat| mat.as_str()).collect();
            
            for m in matches {
                let num = get_number(m);
                if first == 0 {
                    first = num;
                }
                last = num;
            }
        }
       
        let mut s_num = first.to_string().to_owned();
        s_num.push_str(&last.to_string());
        let num = s_num.parse::<i32>().unwrap();
        
        calibration_values.push(num);
    }

    calibration_values.iter().sum()
}

#[cfg(test)]
mod test {
    use super::*;
    const INPUT1: &str = include_str!("input.test1.txt");
    const INPUT2: &str = include_str!("input.test2.txt");

    #[test]
    fn p1() {
        assert_eq!(part1(INPUT1), 142);
    }

    #[test]
    fn p2() {
        assert_eq!(part2(INPUT2), 281);
    }
}
