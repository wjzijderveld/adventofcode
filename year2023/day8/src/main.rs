use std::{collections::HashMap, time::Instant};

fn main() {
    let input = input::get_input_lines();
    let mut lines = input.lines();

    let started_at = Instant::now();

    let directions = lines.next().unwrap();
    lines.next(); // skip empty line

    // dbg!(input.lines().collect::<Vec<&str>>());
    let re = regex::Regex::new(r"(?<node>\w+) = \((?<left>\w+), (?<right>\w+)\)").unwrap();
    let mut map: HashMap<String, Node> = HashMap::new();
    let mut nodes_ending_on_a = vec![];
    for line in lines {
        let captures = re.captures(line).unwrap();
        let node = match (captures.name("node"), captures.name("left"), captures.name("right")) {
            (Some(node), Some(left), Some(right)) => Node::new(node.as_str(), left.as_str(), right.as_str()),
            _ => panic!("invalid map")
        };
        map.insert(node.get_key(), node.clone());
        if node.get_key().ends_with('A') {
            nodes_ending_on_a.push(node);
        }
    }

    input::lap("Parsed input".to_owned(), started_at);

    // part 1 - bruteforce it
    let mut steps: usize = 0;
    let mut on_nodes = nodes_ending_on_a.clone();
    let mut found_nodes_ending_on_z = 0;
    'outer: loop {
        for c in directions.chars() {
            let mut new_nodes: Vec<Node> = vec![];
            for node in on_nodes {
                let next_node = match c {
                    'L' => map.get(&node.left).unwrap(),
                    'R' => map.get(&node.right).unwrap(),
                    _ => panic!("invalid direction {}", c)
                };
                new_nodes.push(next_node.clone());
            }
            steps += 1;
            let ending_in_z: Vec<&Node> = new_nodes.iter().filter(|n| n.get_key().ends_with('Z')).collect();
            found_nodes_ending_on_z = ending_in_z.len();
            if ending_in_z.len() == new_nodes.len() {
                break 'outer;
            }
            on_nodes = new_nodes;
        }

        input::lap(format!("After {} steps we only found {} of the {} nodes ending in Z, starting over", steps, found_nodes_ending_on_z, nodes_ending_on_a.len()), started_at);
    }

    println!("Found nodes ending in Z in {} steps", steps);
}

#[derive(Debug, Clone)]
struct Node {
    key: String,
    left: String,
    right: String,
}

impl Node {
    fn new(key: &str, left: &str, right: &str) -> Self {
        Self {
            key: key.to_owned(),
            left: left.to_owned(),
            right: right.to_owned(),
        }
    }

    fn get_key(&self) -> String {
        self.key.to_owned()
    }
}
