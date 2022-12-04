pub fn run() {
    let data = include_str!("input.txt");

    println!("D2P1 - My Rochambeau total score: {}", part1(&data));
    println!("D2P2 - My Rochambeau score with strategy: {}", part2(&data));
}

fn part1(data: &str) -> isize {
    let rounds: Vec<&str> = data.lines().collect();
    let mut scores = [0, 0];

    for round in rounds {
        let plays: Vec<&str> = round.split_whitespace().collect();

        let round_score = calculate_scores(plays[0], plays[1]);
        scores[0] += round_score[0];
        scores[1] += round_score[1];
    }

    scores[1]
}

fn part2(data: &str) -> isize {
    let rounds: Vec<&str> = data.lines().collect();
    let mut scores = [0, 0];

    for round in rounds {
        let plays: Vec<&str> = round.split_whitespace().collect();

        let round_score = calculate_scores2(plays[0], plays[1]);
        scores[0] += round_score[0];
        scores[1] += round_score[1];
    }

    scores[1]
}

// Assuming X=Lose, Y=Draw, and Z=Win
fn calculate_scores2(theirs: &str, outcome: &str) -> [isize; 2] {
    let their_play: isize = match theirs {
        "A" => 1, //Rock
        "B" => 2, //Paper
        "C" => 3, //Scissors
        _ => 0,
    };

    if outcome == "X" {
        // I must lose
        if their_play == 1 {
            return [their_play + 6, 3];
        }
        [their_play + 6, their_play - 1]
    } else if outcome == "Y" {
        // Draw
        [their_play + 3, their_play + 3]
    } else {
        // I must win
        if their_play == 3 {
            return [their_play, 1 + 6];
        }
        [their_play, their_play + 1 + 6]
    }
}

// Assuming X=Rock, Y=Paper, and Z=Scissors
fn calculate_scores(theirs: &str, mine: &str) -> [isize; 2] {
    let their_play: isize = match theirs {
        "A" => 1,
        "B" => 2,
        "C" => 3,
        _ => 0,
    };
    let my_play: isize = match mine {
        "X" => 1,
        "Y" => 2,
        "Z" => 3,
        _ => 0,
    };

    let diff: isize = (their_play - my_play).try_into().unwrap();

    match diff {
        1 | -2 => [their_play + 6, my_play], // They won
        -1 | 2 => [their_play, my_play + 6], // I won
        _ => [their_play + 3, my_play + 3],  // Draw
    }
}

#[cfg(test)]
mod test {
    use super::*;
    const INPUT: &str = include_str!("input.test.txt");

    #[test]
    fn d2p1() {
        assert_eq!(part1(INPUT), 15);
    }

    #[test]
    fn d2p2() {
        assert_eq!(part2(INPUT), 12);
    }
}
