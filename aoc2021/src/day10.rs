use lazy_static::lazy_static;
use std::collections::HashMap;
use std::fs;

lazy_static! {
    static ref PAIRS: HashMap<&'static str, &'static str> = {
        let mut pairs = HashMap::new();
        pairs.insert("(", ")");
        pairs.insert("[", "]");
        pairs.insert("{", "}");
        pairs.insert("<", ">");
        pairs.insert(")", "(");
        pairs.insert("]", "[");
        pairs.insert("}", "{");
        pairs.insert(">", "<");
        pairs
    };
}

pub fn run() {
    let data = fs::read_to_string("./data/day10.txt").expect("Error reading file!");

    let p1_results = part1(&data);
    println!("Total syntax error score: {}", p1_results);

    // let p2_results = part2(&data);
    // println!("total syntax error score part 2: {}", p2_results);
}

fn part1(data: &str) -> usize {
    let navigation_data: Vec<&str> = parse_data(&data);
    let mut expected: HashMap<&str, usize> = HashMap::new();

    for line in navigation_data.iter() {
        let mut opens: Vec<&str> = Vec::new();
        let chars: Vec<&str> = line.split_terminator("").skip(1).collect::<Vec<&str>>();

        // println!("Line: {:?}", line);
        for v in chars.iter() {
            if is_open(*v) {
                opens.push(*v);
                continue;
            }

            let last_open: &str = opens.pop().unwrap_or("");
            if last_open != *PAIRS.get(*v).unwrap_or(&"") {
                // println!("Expected {:?}, found {}", *PAIRS.get(*v).unwrap_or(&""), last_open);
                *expected.entry(*v).or_insert(0) += 1;
                break;
            }
        }
    }

    calculate_syntax_score(&expected)
}

fn part2(data: &str) -> usize {
    let navigation_data: Vec<&str> = parse_data(&data);
    let mut incomplete: Vec<&str> = Vec::new();

    for line in navigation_data.iter() {
        let mut opens: Vec<&str> = Vec::new();
        let chars: Vec<&str> = line.split_terminator("").skip(1).collect::<Vec<&str>>();

        for v in chars.iter() {
            if is_open(*v) {
                opens.push(*v);
                continue;
            }

            let last_open: &str = opens.pop().unwrap_or("");
            if last_open != *PAIRS.get(*v).unwrap_or(&"") {
                break;
            }
        }

        if opens.len() > 0 {}
    }

    calculate_autocomplete_score(&incomplete)
}

fn parse_data(data: &str) -> Vec<&str> {
    let lines: Vec<&str> = data.lines().collect();

    lines
}

fn is_open(value: &str) -> bool {
    if value == "(" || value == "[" || value == "{" || value == "<" {
        return true;
    }
    false
}

fn calculate_autocomplete_score(data: &Vec<&str>) -> usize {
    let mut point_dictionary: HashMap<&str, usize> = HashMap::new();
    point_dictionary.insert(")", 1);
    point_dictionary.insert("]", 2);
    point_dictionary.insert("}", 3);
    point_dictionary.insert(">", 4);

    let mut score: usize = 0;
    for v in data.iter() {
        score += point_dictionary[*k] * v;
    }
    score
}

fn calculate_syntax_score(data: &HashMap<&str, usize>) -> usize {
    let mut point_dictionary: HashMap<&str, usize> = HashMap::new();
    point_dictionary.insert(")", 3);
    point_dictionary.insert("]", 57);
    point_dictionary.insert("}", 1197);
    point_dictionary.insert(">", 25137);

    let mut score: usize = 0;
    for (k, v) in data.iter() {
        score += point_dictionary[*k] * v;
    }
    score
}

#[cfg(test)]
mod test {
    use super::*;

    const INPUT: &str = "[({(<(())[]>[[{[]{<()<>>
[(()[<>])]({[<{<<[]>>(
{([(<{}[<>[]}>{[]{[(<()>
(((({<>}<{<{<>}{[]{[]{}
[[<[([]))<([[{}[[()]]]
[{[{({}]{}}([{[{{{}}([]
{<[[]]>}<{[{[{[]{()[[[]
[<(<(<(<{}))><([]([]()
<{([([[(<>()){}]>(<<{{
<{([{{}}[<[[[<>{}]]]>[]]";

    #[test]
    fn p1() {
        assert_eq!(part1(INPUT), 26397);
    }

    #[test]
    fn p2() {
        assert_eq!(part2(INPUT), 0);
    }
}
