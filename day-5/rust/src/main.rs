use std::{io, collections::HashMap};

struct Range {
    min: u64,
    max: u64,
}

fn read_input() -> Vec<String> {
    let mut input = Vec::new();
    loop {
        let mut line = String::new();
        match io::stdin().read_line(&mut line) {
            Ok(_) => {
                if line.trim().is_empty() {
                    break;
                }
                input.push(line.trim().to_string());
            }
            Err(error) => println!("error: {}", error),
        }
    }
    input
}

fn parse_input(input: &Vec<String>) -> (Vec<u64>, HashMap<Range, u64>){
    
}

main() {
    let input = read_input();
    let parsed = parse_input(&input);
}
