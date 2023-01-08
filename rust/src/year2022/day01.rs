const DATA: &str = include_str!("../../data/year2022/day01.txt");

pub fn run_part1() {
    let input = parse_data(DATA);
    println!("most calories: {}", get_most_calories(&input));
}

pub fn run_part2() {
    let input = parse_data(DATA);
    println!("top 3: {}", sum_top_three_calories(&input));
}

fn parse_data(data: &str) -> Vec<Vec<i32>> {
    data.split("\n\n")
        .map(|s| {
            s.lines()
                .map(|l| l.parse::<i32>().unwrap())
                .collect::<Vec<_>>()
        })
        .collect()
}

fn get_most_calories(input: &Vec<Vec<i32>>) -> i32 {
    input.iter().map(|elf| elf.iter().sum()).max().unwrap()
}

fn sum_top_three_calories(input: &Vec<Vec<i32>>) -> i32 {
    let mut elves = input
        .iter()
        .map(|elf| elf.iter().sum())
        .collect::<Vec<i32>>();
    elves.sort();
    elves.iter().rev().take(3).sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    const TEST_DATA: &str = "\
1000
2000
3000

4000

5000
6000

7000
8000
9000

10000";

    #[test]
    fn test_get_most_calories() {
        let input = parse_data(TEST_DATA);
        let calories = get_most_calories(&input);
        assert_eq!(calories, 24000);
    }

    #[test]
    fn test_sum_top_three_calories() {
        let input = parse_data(TEST_DATA);
        let calories = sum_top_three_calories(&input);
        assert_eq!(calories, 45000);
    }
}
