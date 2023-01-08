const DATA: &str = include_str!("../../data/year2022/day02.txt");

pub fn run_part1() {
    let input = parse_data(DATA);
    let result = score_following_strategy_guide(&input);
    println!("Result: {}", result);
}

pub fn run_part2() {
    let input = parse_data(DATA);
    let result = score_correct_strategy_guide(&input);
    println!("Result: {}", result);
}

fn score_following_strategy_guide(input: &[(&str, &str)]) -> i32 {
    input
        .iter()
        .map(|(opp, player)| round_score(opp, player))
        .sum()
}

fn score_correct_strategy_guide(input: &[(&str, &str)]) -> i32 {
    input
        .iter()
        .map(|(opp, player)| correct_round_scoring(opp, player))
        .sum()
}

fn round_score(opp: &str, player: &str) -> i32 {
    let shape = match player {
        "X" => 1,
        "Y" => 2,
        "Z" => 3,
        _ => unreachable!(),
    };
    let outcome = match (opp, player) {
        ("A", "X") | ("B", "Y") | ("C", "Z") => 3,
        ("A", "Y") | ("B", "Z") | ("C", "X") => 6,
        _ => 0,
    };
    shape + outcome
}

fn correct_round_scoring(opp: &str, outcome: &str) -> i32 {
    match outcome {
        "X" => calc_loss(opp),
        "Y" => calc_draw(opp),
        "Z" => calc_win(opp),
        _ => unreachable!(),
    }
}

fn calc_loss(opp: &str) -> i32 {
    0 + match opp {
        "A" => 3,
        "B" => 1,
        "C" => 2,
        _ => unreachable!(),
    }
}

fn calc_draw(opp: &str) -> i32 {
    3 + match opp {
        "A" => 1,
        "B" => 2,
        "C" => 3,
        _ => unreachable!(),
    }
}

fn calc_win(opp: &str) -> i32 {
    6 + match opp {
        "A" => 2,
        "B" => 3,
        "C" => 1,
        _ => unreachable!(),
    }
}

fn parse_data(data: &str) -> Vec<(&str, &str)> {
    data.lines()
        .map(|line| line.split(' ').collect::<Vec<_>>())
        .map(|line| (line[0], line[1]))
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    const TEST_DATA: &str = "\
A Y
B X
C Z";

    #[test]
    fn test_parse_data() {
        let res = parse_data(TEST_DATA);
        let expected = vec![("A", "Y"), ("B", "X"), ("C", "Z")];
        assert_eq!(res, expected);
    }

    #[test]
    fn test_score_following_strategy_guide() {
        let input = parse_data(TEST_DATA);
        let result = score_following_strategy_guide(&input);
        assert_eq!(result, 15);
    }

    #[test]
    fn test_score_correct_strategy_guide() {
        let input = parse_data(TEST_DATA);
        let result = score_correct_strategy_guide(&input);
        assert_eq!(result, 12);
    }
}
