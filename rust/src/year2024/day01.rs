const DATA: &str = include_str!("../../data/year2024/day01.txt");

pub fn run_part1() {
    let input = parse_data(DATA);
    let distance = get_total_distance(input);
    println!("Total distance: {}", distance);
}

fn parse_data(data: &str) -> (Vec<usize>, Vec<usize>) {
    data.lines()
        .map(|l| {
            let mut parts = l.split_whitespace();
            (
                parts.next().unwrap().parse::<usize>().unwrap(),
                parts.next().unwrap().parse::<usize>().unwrap(),
            )
        })
        .collect()
}

fn get_total_distance(mut input: (Vec<usize>, Vec<usize>)) -> usize {
    input.0.sort();
    input.1.sort();
    let a = input
        .0
        .into_iter()
        .zip(input.1.into_iter())
        .map(|(left, right)| left.abs_diff(right));
    a.sum::<usize>()
}

#[cfg(test)]
mod tests {
    use super::*;

    const TEST_DATA: &str = "\
3   4
4   3
2   5
1   3
3   9
3   3";

    #[test]
    fn test_parse_data() {
        let input = parse_data(TEST_DATA);
        assert_eq!(input, (vec![3, 4, 2, 1, 3, 3], vec![4, 3, 5, 3, 9, 3]));
    }

    #[test]
    fn test_get_total_distance() {
        let input = parse_data(TEST_DATA);
        let distance = get_total_distance(input);
        assert_eq!(distance, 11);
    }
}
