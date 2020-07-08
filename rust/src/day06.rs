use std::fs;

pub fn run() {
    let raw =
        fs::read_to_string("/Users/hanhossain/Developer/advent-of-code/data/Day06.txt").unwrap();
    let tree = Tree::parse(&raw);

    let total_orbits = tree.total_orbits();
    println!("Total orbits: {}", total_orbits);
}

#[derive(Debug)]
struct Node {
    id: String,
    orbits: i32,
    children: Vec<Box<Node>>,
}

impl Node {
    fn new(id: String, orbits: i32) -> Self {
        Self {
            children: vec![],
            orbits,
            id,
        }
    }

    fn search_children(&mut self, id: &str) -> Option<&mut Box<Self>> {
        for child in &mut self.children {
            if child.id == id {
                return Some(child);
            }
            if let Some(x) = child.search_children(id) {
                return Some(x);
            }
        }

        None
    }

    fn total_orbits(&self) -> i32 {
        let mut orbits = self.orbits;

        for child in &self.children {
            orbits += child.total_orbits();
        }

        orbits
    }
}

#[derive(Debug)]
struct Tree {
    root: Box<Node>,
}

impl Tree {
    fn parse(data: &str) -> Self {
        let mut line_iterator = data.lines();

        let first_line = line_iterator.next().unwrap();
        let mut iterator = first_line.trim().split(")");
        let planet = iterator.next().unwrap();
        let satellite = iterator.next().unwrap().to_owned();
        let mut root = Box::new(Node::new(planet.to_owned(), 0));
        let satellite = Box::new(Node::new(satellite, root.orbits + 1));
        root.children.push(satellite);

        for line in line_iterator {
            let mut iterator = line.trim().split(")");
            let planet = iterator.next().unwrap();
            let satellite = iterator.next().unwrap().to_owned();

            let planet = if root.id == planet {
                &mut root
            } else {
                root.search_children(planet)
                    .expect(&format!("couldn't find planet {}", planet))
            };

            let satellite = Box::new(Node::new(satellite, planet.orbits + 1));
            planet.children.push(satellite);
        }

        Self { root }
    }

    fn search(&mut self, id: &str) -> &mut Box<Node> {
        if self.root.id == id {
            return &mut self.root;
        }

        self.root
            .search_children(id)
            .expect(&format!("couldn't find the node {}", id))
    }

    fn total_orbits(&self) -> i32 {
        self.root.total_orbits()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const DATA: &str = r#"COM)B
B)C
C)D
D)E
E)F
B)G
G)H
D)I
E)J
J)K
K)L"#;

    #[test]
    fn test_parse_single_satellite() {
        let tree = Tree::parse("COM)B");

        assert_eq!(tree.root.id, "COM");
        assert_eq!(tree.root.children[0].id, "B");
    }

    #[test]
    fn test_parse_multiple_satellites() {
        let _tree = Tree::parse(DATA);
    }

    #[test]
    fn test_total_orbits() {
        let tree = Tree::parse(DATA);

        assert_eq!(tree.total_orbits(), 42);
    }
}
