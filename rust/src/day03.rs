pub fn run_part1() {
    read_file();
}

fn read_file() {
    // let filepath = "/Users/hanhossain/Developer/advent-of-code/data/Day03.txt";

    // fs::read_to_string(filepath)
    //    .unwrap()
    //    .split(",")
    //    .flat_map(|x| x.parse::<i32>())
    //    .collect()

    let s = r#"R75,D30,R83,U83,L12,D49,R71,U7,L72
U62,R66,U55,R34,D71,R55,D58,R83"#;

    let mut a = Vec::new();
    for line in s.lines() {
        let directions = line.split(",").map(|x| Direction::parse(x)).collect::<Vec<Direction>>();
        a.push(directions);
    }

    println!("{:#?}", a);
}

#[derive(Debug)]
enum Direction {
    Up(usize),
    Down(usize),
    Left(usize),
    Right(usize),
}

impl Direction {
    fn parse(s: &str) -> Self {
        match (&s[..1], &s[1..]) {
            ("U", x) => Direction::Up(x.parse::<usize>().unwrap()),
            ("D", x) => Direction::Down(x.parse::<usize>().unwrap()),
            ("L", x) => Direction::Left(x.parse::<usize>().unwrap()),
            ("R", x) => Direction::Right(x.parse::<usize>().unwrap()),
            _ => unreachable!()
        }
    }
}



