use std::collections::HashMap;

const DATA: &str = include_str!("../../data/year2024/day01.txt");

pub fn run_part1() {
    let input = parse_data(DATA);
    let distance = get_total_distance(input);
    println!("Total distance: {}", distance);
}

pub fn run_part2() {
    let input = parse_data(DATA);
    let similarity_score = get_similarity_score(input);
    println!("Similarity score: {}", similarity_score);
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

fn get_similarity_score(input: (Vec<usize>, Vec<usize>)) -> usize {
    let left = count_instances(&input.0);
    let right = count_instances(&input.1);
    left.into_iter()
        .map(|(k, v)| k * v * right.get(&k).unwrap_or(&0))
        .sum()
}

fn count_instances(input: &Vec<usize>) -> HashMap<usize, usize> {
    input.iter().fold(HashMap::new(), |mut acc, &x| {
        *acc.entry(x).or_insert(0) += 1;
        acc
    })
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

    #[test]
    fn test_get_similarity_score() {
        let input = parse_data(TEST_DATA);
        let similarity_score = get_similarity_score(input);
        assert_eq!(similarity_score, 31);
    }
}
