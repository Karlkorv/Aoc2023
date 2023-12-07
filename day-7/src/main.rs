use std::collections::HashMap;
use std::io::stdin;

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

fn parse_input(input: &Vec<String>) -> Vec<(i32, i32)> {
    let mut parsed = Vec::new();
    for line in input {
        let mut parts = line.split(" ");
        let card = parts.next().unwrap().to_string();
        let card_value: i32 = calc_card_value(&card);
        let bid: i32 = parts.next().unwrap().to_string().parse().unwrap();
        parsed.push((card_value, bid));
    }
    parsed
}

fn calc_card_value(card: &String) -> i32 {
    let mut value: i32 = 0;
    value |= get_suit_value(&card) << 20;
    value |= get_rank_value(&card);
    value
}

// part one
// fn get_rank_value(card: &String) -> i32 {
//     let mut total_value = 0;
//     let mut count = 4;
//     for c in card.chars() {
//         let value: i32 = match c {
//             'A' => 14,
//             'K' => 13,
//             'Q' => 12,
//             'J' => 11,
//             'T' => 10,
//             _ => c.to_string().parse().unwrap(),
//         };
//         total_value |= value << (4 * count);
//         count -= 1;
//     }
//     total_value
// }
//
// fn get_suit_value(card: &String) -> i32 {
//     let mut counts: HashMap<char, i32> = HashMap::new();

//     for c in card.chars() {
//         let count = counts.entry(c).or_insert(0);
//         *count += 1;
//     }

//     let mut ident: Vec<i32> = counts.into_values().collect();
//     ident.sort();

//     match ident.as_slice() {
//         [1, 1, 1, 1, 1] => 0,
//         [1, 1, 1, 2] => 1,
//         [1, 2, 2] => 2,
//         [1, 1, 3] => 3,
//         [2, 3] => 4,
//         [1, 4] => 5,
//         [5] => 6,
//         _ => panic!("Invalid card: {}", card),
//     }
// }

// part two

fn get_rank_value(card: &String) -> i32 {
    let mut total_value = 0;
    let mut count = 4;
    for c in card.chars() {
        let value: i32 = match c {
            'A' => 14,
            'K' => 13,
            'Q' => 12,
            'J' => 1,
            'T' => 10,
            _ => c.to_string().parse().unwrap(),
        };
        total_value |= value << (4 * count);
        count -= 1;
    }
    total_value
}

fn get_suit_value(card: &String) -> i32 {
    let mut counts: HashMap<char, i32> = HashMap::new();
    let mut joker_count = 0;

    for c in card.chars() {
        if c == 'J' {
            joker_count += 1;
            continue;
        }
        let count = counts.entry(c).or_insert(0);
        *count += 1;
    }

    let mut ident: Vec<i32> = counts.into_values().collect();
    ident.sort();

    match ident.as_mut_slice() {
        [1, 1, 1, 1, 1] => 0,
        [1, 1, 1, 2] => 1,
        [1, 2, 2] => 2,
        [1, 1, 3] => 3,
        [2, 3] => 4,
        [1, 4] => 5,
        [5] => 6,
        _ => joker_type(joker_count, ident),
    }
}

fn joker_type(joker_count: i32, mut ident: Vec<i32>) -> i32 {
    if joker_count == 5 {
        return 6;
    }
    *ident.last_mut().unwrap() += joker_count;
    match ident.as_slice() {
        [1, 1, 1, 1, 1] => 0,
        [1, 1, 1, 2] => 1,
        [1, 2, 2] => 2,
        [1, 1, 3] => 3,
        [2, 3] => 4,
        [1, 4] => 5,
        [5] => 6,
        _ => panic!("Invalid card: {:?}", ident),
    }
}

fn solve(parsed: Vec<(i32, i32)>) -> i32 {
    let mut new_parsed = parsed.clone();
    new_parsed.sort_by(|a, b| a.0.cmp(&b.0));
    let mut result = 0;

    for i in 0..new_parsed.len() {
        let (_, bid) = new_parsed[i];
        result += bid * (i as i32 + 1);
    }

    result
}

fn main() {
    let input = read_input();
    let parsed = parse_input(&input);
    let result = solve(parsed);
    println!("{}", result);
}
