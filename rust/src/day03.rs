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

    // s.split(",").flat_map(|x| x.parse::<i32>())
    // let a: Vec<&str> = s.lines().split(",").collect();
    for line in s.lines() {
        println!("{:?}", line);
        println!("{:?}", line.split(",").collect::<Vec<&str>>());
    }
    Direction::parse("R32");
}

#[derive(Debug)]
enum Direction {
    Up(usize),
    Down(usize),
    Left(usize),
    Right(usize),
}

impl Direction {
    fn parse(s: &str) {
        println!("{}", &s[..1]);
    }
}



