use regex::Regex;
use std::{collections::HashMap, io::stdin};

struct Graph {
    edges: HashMap<String, (String, String)>, // 0 left, 1 right
}

impl Graph {
    fn new() -> Self {
        Graph {
            edges: HashMap::new(),
        }
    }

    fn get_edge(&self, start: &str) -> (String, String) {
        self.edges[start].clone()
    }

    fn add_edge(&mut self, start: String, left: String, right: String) {
        self.edges.insert(start, (left, right));
    }
}

fn read_input() -> Vec<String> {
    let mut input = Vec::new();
    loop {
        let mut line = String::new();
        let bytes_read = stdin().read_line(&mut line).unwrap();
        if bytes_read == 0 {
            break;
        }
        input.push(line.trim().to_string());
    }
    input
}

fn parse_input(input: &Vec<String>) -> (String, Graph) {
    let path: String = input[0].clone();
    let re = Regex::new(r"(\w+)").unwrap();
    let mut graph = Graph::new();
    for line in input[2..].iter() {
        let caps = re.captures_iter(line).collect::<Vec<_>>();
        let start = caps[0][0].to_string();
        let left = caps[1][0].to_string();
        let right = caps[2][0].to_string();
        graph.add_edge(start, left, right);
    }
    (path, graph)
}

// part one
// fn solve((path, graph): &(String, Graph)) -> i32 {
//     let mut count = 0;
//     let mut current: String = "AAA".to_string();

//     while current != "ZZZ" {
//         for c in path.chars() {
//             count += 1;
//             current = match c {
//                 'L' => graph.get_edge(current).0,
//                 'R' => graph.get_edge(current).1,
//                 _ => panic!("Invalid path"),
//             };
//             if current == "ZZZ" {
//                 break;
//             }
//         }
//     }

//     count
// }

fn solve((path, graph): &(String, Graph)) -> usize {
    let current: Vec<String> = graph
        .edges
        .keys()
        .filter(|k| k.ends_with("A"))
        .map(|k| k.to_string())
        .collect();
    let mut count: Vec<i32> = vec![];

    for pos in current.iter() {
        let mut counter = 0;
        let mut pos = pos.clone();
        let mut z_location = 0;
        for c in path.chars().cycle() {
            counter += 1;
            pos = match c {
                'L' => graph.get_edge(&pos).0,
                'R' => graph.get_edge(&pos).1,
                _ => panic!("Invalid path"),
            };
            if pos.ends_with("Z") {
                if z_location == 0 {
                    z_location = counter;
                } else {
                    count.push(counter - z_location);
                    break;
                }
            }
        }
    }

    // cast count into usize slice
    let count: Vec<usize> = count.iter().map(|x| *x as usize).collect();
    println!("{:?}", count);

    let result = lcm(&count);

    result
}

// Från nätet: https://github.com/TheAlgorithms/Rust/blob/master/src/math/lcm_of_n_numbers.rs
pub fn lcm(nums: &[usize]) -> usize {
    if nums.len() == 1 {
        return nums[0];
    }
    let a = nums[0];
    let b = lcm(&nums[1..]);
    a * b / gcd_of_two_numbers(a, b)
}

fn gcd_of_two_numbers(a: usize, b: usize) -> usize {
    if b == 0 {
        return a;
    }
    gcd_of_two_numbers(b, a % b)
}

fn main() {
    let input = read_input();
    let parsed = parse_input(&input);
    let result = solve(&parsed);
    println!("{}", result);
}
