use std::collections::{HashMap, HashSet};

const DATA: &str = include_str!("../../data/year2019/day06.txt");

pub fn run_part1() {
    let tree = Tree::parse(DATA);

    let total_orbits = tree.total_orbits();
    println!("Total orbits: {}", total_orbits);
}

pub fn run_part2() {
    let tree = Tree::parse(DATA);

    let transfers = tree.orbital_transfers("YOU", "SAN");
    println!("Orbital transfers: {}", transfers);
}

#[derive(Debug, Eq, PartialEq)]
struct Node {
    id: String,
    parent: Option<String>,
    children: Vec<String>,
}

impl Node {
    fn new(id: String) -> Self {
        Self {
            id,
            parent: None,
            children: Vec::new(),
        }
    }
}

#[derive(Debug, Eq, PartialEq)]
struct Tree {
    nodes: HashMap<String, Node>,
}

impl Tree {
    fn parse(data: &str) -> Self {
        let mut tree = Self {
            nodes: HashMap::new(),
        };

        for line in data.lines().map(|x| x.trim()) {
            let mut iterator = line.split(")");
            let planet_id = iterator.next().unwrap();
            let satellite = iterator.next().unwrap();

            tree.ensure_planet_exists(planet_id);
            tree.ensure_planet_exists(satellite);

            let planet = tree.nodes.get_mut(planet_id).unwrap();
            planet.children.push(satellite.to_owned());

            let satellite = tree.nodes.get_mut(satellite).unwrap();
            satellite.parent = Some(planet_id.to_owned());
        }

        tree
    }

    fn ensure_planet_exists(&mut self, id: &str) {
        if !self.nodes.contains_key(id) {
            let planet = Node::new(id.to_owned());
            self.nodes.insert(id.to_owned(), planet);
        }
    }

    fn total_orbits(&self) -> i32 {
        let mut total = 0;
        let mut nodes = vec![(self.nodes.get("COM").unwrap(), 0)];

        // basically a DFS
        while let Some((node, orbits)) = nodes.pop() {
            for child in &node.children {
                let child = self.nodes.get(child).unwrap();
                nodes.push((child, orbits + 1));
            }

            total += orbits;
        }

        total
    }

    fn path(&self, id: &str) -> Vec<String> {
        let mut path = Vec::new();

        let mut node = self.nodes.get(id).unwrap();
        while let Some(parent) = &node.parent {
            path.push(parent.to_owned());
            node = self.nodes.get(parent).unwrap();
        }

        path
    }

    fn orbital_transfers(&self, source: &str, destination: &str) -> usize {
        let you: HashSet<String> = self.path(source).into_iter().collect();
        let san: HashSet<String> = self.path(destination).into_iter().collect();
        you.symmetric_difference(&san)
            .collect::<HashSet<&String>>()
            .len()
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

    const DATA2: &str = r#"COM)B
B)C
C)D
D)E
E)F
B)G
G)H
D)I
E)J
J)K
K)L
K)YOU
I)SAN"#;

    #[test]
    fn test_parse_single_satellite() {
        let tree = Tree::parse("COM)B");
        println!("{:#?}", tree);

        let mut expected: HashMap<String, Node> = HashMap::new();
        expected.insert(
            "COM".to_owned(),
            Node {
                id: "COM".to_owned(),
                parent: None,
                children: vec!["B".to_owned()],
            },
        );

        expected.insert(
            "B".to_owned(),
            Node {
                id: "B".to_owned(),
                parent: Some("COM".to_owned()),
                children: vec![],
            },
        );

        let expected = Tree { nodes: expected };
        assert_eq!(tree, expected);
    }

    #[test]
    fn test_parse_multiple_satellites() {
        let tree = Tree::parse(DATA);
        println!("{:#?}", tree);
    }

    #[test]
    fn test_total_orbits() {
        let tree = Tree::parse(DATA);

        assert_eq!(tree.total_orbits(), 42);
    }

    #[test]
    fn test_orbital_transfers() {
        let tree = Tree::parse(DATA2);
        let transfers = tree.orbital_transfers("YOU", "SAN");

        assert_eq!(transfers, 4);
    }
}
