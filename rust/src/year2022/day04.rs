const DATA: &str = include_str!("../../data/year2022/day04.txt");

pub fn run_part1() {
    let data = parse_data(DATA);
    let count = fully_contained_pair_count(&data);
    println!("fully contained: {}", count);
}

pub fn run_part2() {
    let data = parse_data(DATA);
    let count = overlap_count(&data);
    println!("overlap: {}", count);
}

fn fully_contained_pair_count(pairs: &[((i32, i32), (i32, i32))]) -> usize {
    pairs
        .iter()
        .filter(|(elf1, elf2)| is_fully_contained(*elf1, *elf2))
        .count()
}

fn overlap_count(pairs: &[((i32, i32), (i32, i32))]) -> usize {
    pairs
        .iter()
        .filter(|(elf1, elf2)| has_overlap(*elf1, *elf2))
        .count()
}

fn is_fully_contained(elf1: (i32, i32), elf2: (i32, i32)) -> bool {
    (elf1.0 <= elf2.0 && elf1.1 >= elf2.1) || (elf2.0 <= elf1.0 && elf2.1 >= elf1.1)
}

fn has_overlap(elf1: (i32, i32), elf2: (i32, i32)) -> bool {
    let (l1, h1) = elf1;
    let (l2, h2) = elf2;
    // l1 l2 h1
    (l1 <= l2 && l2 <= h1)
    // l1 h2 h1
    || (l1 <= h2 && h2 <= h1)
    // l2 l1 h2
    || (l2 <= l1 && l1 <= h2)
    // l2 h1 h2
    || (l2 <= h1 && h1 <= h2)
}

fn parse_data(data: &str) -> Vec<((i32, i32), (i32, i32))> {
    data.lines()
        .map(|l| {
            l.split(|c| c == '-' || c == ',')
                .map(|c| c.parse().unwrap())
                .collect::<Vec<_>>()
        })
        .map(|l| ((l[0], l[1]), (l[2], l[3])))
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::assert_eq;

    const TEST_DATA: &str = "\
2-4,6-8
2-3,4-5
5-7,7-9
2-8,3-7
6-6,4-6
2-6,4-8";

    #[test]
    fn test_is_fully_contained() {
        assert!(!is_fully_contained((2, 4), (6, 8)));
        assert!(is_fully_contained((2, 8), (3, 7)));
        assert!(is_fully_contained((6, 6), (4, 6)));
    }

    #[test]
    fn test_fully_contained_pair_count() {
        let pairs = parse_data(TEST_DATA);
        assert_eq!(fully_contained_pair_count(&pairs), 2);
    }

    #[test]
    fn test_has_overlap() {
        assert!(has_overlap((5, 7), (7, 9)));
        assert!(has_overlap((2, 8), (3, 7)));
        assert!(has_overlap((6, 6), (4, 6)));
        assert!(has_overlap((2, 6), (4, 8)));
    }

    #[test]
    fn test_overlap_count() {
        let data = parse_data(TEST_DATA);
        assert_eq!(overlap_count(&data), 4);
    }
}
