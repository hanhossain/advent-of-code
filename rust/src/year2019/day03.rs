use std::collections::{HashMap, HashSet};

const DATA: &str = include_str!("../../data/year2019/day03.txt");

pub fn run_part1() {
    let distance = calculate_distance(DATA);
    println!("Distance: {}", distance);
}

pub fn run_part2() {
    let signal_delay = calculate_signal_delay(DATA);
    println!("Signal delay: {}", signal_delay);
}

fn parse(s: &str) -> Vec<Trail> {
    s.lines()
        .map(|l| {
            let segments = l
                .split(",")
                .map(|x| Segment::parse(x))
                .collect::<Vec<Segment>>();
            let mut trail = Trail::new();
            trail.update(&segments);
            trail
        })
        .collect()
}

fn calculate_distance(s: &str) -> i32 {
    let trails = parse(s);
    let trail1 = &trails[0];
    let trail2 = &trails[1];

    trail1
        .coordinates
        .keys()
        .collect::<HashSet<_>>()
        .intersection(&trail2.coordinates.keys().collect::<HashSet<_>>())
        .map(|(x, y)| x.abs() + y.abs())
        .min()
        .unwrap()
}

fn calculate_signal_delay(s: &str) -> i32 {
    let trails = parse(s);
    let trail1 = &trails[0];
    let trail2 = &trails[1];

    trail1
        .coordinates
        .keys()
        .collect::<HashSet<_>>()
        .intersection(&trail2.coordinates.keys().collect::<HashSet<_>>())
        .map(|x| {
            let delay1 = trail1.coordinates.get(x).unwrap().abs();
            let delay2 = trail2.coordinates.get(x).unwrap().abs();
            delay1 + delay2
        })
        .min()
        .unwrap()
}

#[derive(Debug)]
struct Segment {
    x: i32,
    y: i32,
}

impl Segment {
    fn parse(s: &str) -> Self {
        match (&s[..1], &s[1..]) {
            ("U", x) => Segment {
                x: 0,
                y: x.parse::<i32>().unwrap(),
            },
            ("D", x) => Segment {
                x: 0,
                y: -x.parse::<i32>().unwrap(),
            },
            ("L", x) => Segment {
                x: -x.parse::<i32>().unwrap(),
                y: 0,
            },
            ("R", x) => Segment {
                x: x.parse::<i32>().unwrap(),
                y: 0,
            },
            _ => unreachable!(),
        }
    }
}

#[derive(Debug)]
struct Trail {
    last_x: i32,
    last_y: i32,
    coordinates: HashMap<(i32, i32), i32>,
    count: i32,
}

impl Trail {
    fn new() -> Self {
        Self {
            last_x: 0,
            last_y: 0,
            coordinates: HashMap::new(),
            count: 0,
        }
    }

    fn update(&mut self, segments: &[Segment]) {
        for segment in segments {
            if segment.x > 0 {
                for x in 1..=segment.x {
                    self.count += 1;
                    let location = (self.last_x + x, self.last_y);
                    if !self.coordinates.contains_key(&location) {
                        self.coordinates.insert(location, self.count);
                    }
                }
            } else if segment.x < 0 {
                for x in (segment.x..0).rev() {
                    self.count += 1;
                    let location = (self.last_x + x, self.last_y);
                    if !self.coordinates.contains_key(&location) {
                        self.coordinates.insert(location, self.count);
                    }
                }
            }

            if segment.y > 0 {
                for y in 1..=segment.y {
                    self.count += 1;
                    let location = (self.last_x, self.last_y + y);
                    if !self.coordinates.contains_key(&location) {
                        self.coordinates.insert(location, self.count);
                    }
                }
            } else if segment.y < 0 {
                for y in (segment.y..0).rev() {
                    self.count += 1;
                    let location = (self.last_x, self.last_y + y);
                    if !self.coordinates.contains_key(&location) {
                        self.coordinates.insert(location, self.count);
                    }
                }
            }

            self.last_x += segment.x;
            self.last_y += segment.y;
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_calculate_distance1() {
        let raw = r#"R8,U5,L5,D3
U7,R6,D4,L4"#;
        let distance = calculate_distance(raw);
        assert_eq!(distance, 6);
    }

    #[test]
    fn test_calculate_distance2() {
        let raw = r#"R75,D30,R83,U83,L12,D49,R71,U7,L72
U62,R66,U55,R34,D71,R55,D58,R83"#;
        let distance = calculate_distance(raw);
        assert_eq!(distance, 159);
    }

    #[test]
    fn test_calculate_distance3() {
        let raw = r#"R98,U47,R26,D63,R33,U87,L62,D20,R33,U53,R51
U98,R91,D20,R16,D67,R40,U7,R15,U6,R7"#;
        let distance = calculate_distance(raw);
        assert_eq!(distance, 135);
    }

    #[test]
    fn test_calculate_signal_delay1() {
        let raw = r#"R8,U5,L5,D3
U7,R6,D4,L4"#;
        let distance = calculate_signal_delay(raw);
        assert_eq!(distance, 30);
    }

    #[test]
    fn test_calculate_signal_delay2() {
        let raw = r#"R75,D30,R83,U83,L12,D49,R71,U7,L72
U62,R66,U55,R34,D71,R55,D58,R83"#;
        let distance = calculate_signal_delay(raw);
        assert_eq!(distance, 610);
    }

    #[test]
    fn test_calculate_signal_delay3() {
        let raw = r#"R98,U47,R26,D63,R33,U87,L62,D20,R33,U53,R51
U98,R91,D20,R16,D67,R40,U7,R15,U6,R7"#;
        let distance = calculate_signal_delay(raw);
        assert_eq!(distance, 410);
    }
}
