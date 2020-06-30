use std::collections::HashSet;
use std::fs;

pub fn run_part1() {
    let trails = read_file();
    let trail1 = &trails[0];
    let trail2 = &trails[1];
    let distance = get_distance(trail1, trail2);
    println!("Distance: {}", distance);
}

fn read_file() -> Vec<Trail> {
    let filepath = "/Users/hanhossain/Developer/advent-of-code/data/Day03.txt";
    let s = fs::read_to_string(filepath).unwrap();
    parse(&s)
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

fn get_distance(trail1: &Trail, trail2: &Trail) -> i32 {
    trail1
        .coordinates
        .intersection(&trail2.coordinates)
        .map(|(x, y)| x.abs() + y.abs())
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
    coordinates: HashSet<(i32, i32)>,
}

impl Trail {
    fn new() -> Self {
        Self {
            last_x: 0,
            last_y: 0,
            coordinates: HashSet::new(),
        }
    }

    fn update(&mut self, segments: &[Segment]) {
        for segment in segments {
            if segment.x > 0 {
                for x in 1..=segment.x {
                    self.coordinates.insert((self.last_x + x, self.last_y));
                }
            } else if segment.x < 0 {
                for x in (segment.x..0).rev() {
                    self.coordinates.insert((self.last_x + x, self.last_y));
                }
            }

            if segment.y > 0 {
                for y in 1..=segment.y {
                    self.coordinates.insert((self.last_x, self.last_y + y));
                }
            } else if segment.y < 0 {
                for y in (segment.y..0).rev() {
                    self.coordinates.insert((self.last_x, self.last_y + y));
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
    fn test_get_distance1() {
        let raw = r#"R75,D30,R83,U83,L12,D49,R71,U7,L72
U62,R66,U55,R34,D71,R55,D58,R83"#;
        let trails = parse(raw);
        let trail1 = &trails[0];
        let trail2 = &trails[1];
        let distance = get_distance(trail1, trail2);
        assert_eq!(distance, 159);
    }

    #[test]
    fn test_get_distance2() {
        let raw = r#"R98,U47,R26,D63,R33,U87,L62,D20,R33,U53,R51
U98,R91,D20,R16,D67,R40,U7,R15,U6,R7"#;
        let trails = parse(raw);
        let trail1 = &trails[0];
        let trail2 = &trails[1];
        let distance = get_distance(trail1, trail2);
        assert_eq!(distance, 135);
    }
}
