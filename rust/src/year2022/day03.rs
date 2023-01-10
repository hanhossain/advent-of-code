use std::collections::HashSet;

const DATA: &str = include_str!("../../data/year2022/day03.txt");

pub fn run_part1() {
    let priority: i32 = DATA.lines().map(|x| rucksack_priority(x)).sum();
    println!("priority: {}", priority);
}
pub fn run_part2() {
    let mut iter = DATA.lines();
    let mut priorities = 0;

    while let Some(line) = iter.next() {
        let mut group = Vec::new();
        group.push(line);
        let line = iter.next().unwrap();
        group.push(line);
        let line = iter.next().unwrap();
        group.push(line);
        let item_type = badge_item_type(&group);
        priorities += get_priority(item_type);
    }

    println!("Priorities: {}", priorities);
}

fn get_priority(item: char) -> i32 {
    match item {
        'a'..='z' => item as i32 - 97 + 1,
        'A'..='Z' => item as i32 - 65 + 27,
        _ => unreachable!(),
    }
}

fn rucksack_priority(rucksack: &str) -> i32 {
    let length = rucksack.chars().count() / 2;
    let mut existing = HashSet::new();
    for (index, item) in rucksack.char_indices() {
        if index < length {
            existing.insert(item);
        } else {
            if existing.get(&item).is_some() {
                return get_priority(item);
            }
        }
    }
    unreachable!()
}

fn badge_item_type(group: &[&str]) -> char {
    let set1 = group[0].chars().collect::<HashSet<_>>();
    let set2 = group[1].chars().collect::<HashSet<_>>();
    let set3 = group[2].chars().collect::<HashSet<_>>();

    let intersection = set1
        .intersection(&set2)
        .copied()
        .collect::<HashSet<_>>()
        .intersection(&set3)
        .copied()
        .collect::<HashSet<_>>();
    assert_eq!(intersection.len(), 1);

    intersection.into_iter().next().unwrap()
}

#[cfg(test)]
mod tests {
    use super::*;

    const TEST_DATA: &str = "\
vJrwpWtwJgWrhcsFMMfFFhFp
jqHRNqRjqzjGDLGLrsFMfFZSrLrFZsSL
PmmdzqPrVvPwwTWBwg
wMqvLMZHhHMvwLHjbvcjnnSBnvTQFn
ttgJtRGJQctTZtZT
CrZsJsPPZsGzwwsLwLmpwMDw";

    #[test]
    fn test_get_priority() {
        assert_eq!(get_priority('a'), 1);
        assert_eq!(get_priority('c'), 3);
        assert_eq!(get_priority('z'), 26);
        assert_eq!(get_priority('A'), 27);
        assert_eq!(get_priority('Z'), 52);
    }

    #[test]
    fn test_rucksack_priority() {
        let actual = TEST_DATA
            .lines()
            .map(|x| rucksack_priority(x))
            .collect::<Vec<_>>();
        let expected = vec![16, 38, 42, 22, 20, 19];
        assert_eq!(actual, expected);
    }

    #[test]
    fn test_badge_item_type() {
        let group = TEST_DATA.lines().take(3).collect::<Vec<_>>();
        let expected = badge_item_type(&group);
        assert_eq!(expected, 'r');
    }
}
