fn read_input() -> Vec<String> {
    let mut input = Vec::new();
    loop {
        let mut line = String::new();
        let bytes_read = std::io::stdin().read_line(&mut line).expect("I/O error");
        if bytes_read == 0 {
            break;
        }
        input.push(line.trim().to_string());
    }
    input
}

fn parse_input(input: &Vec<String>) -> Vec<Vec<i32>> {
    let out = input
        .iter()
        .map(|line| {
            line.split(" ")
                .map(|word| word.parse::<i32>().unwrap())
                .collect()
        })
        .collect::<Vec<Vec<i32>>>();

    out
}

fn solve(input: &Vec<Vec<i32>>) -> i32 {
    let mut out = Vec::new();
    for line in input {
        out.push(line.last().unwrap() + get_next(line));
    }

    out.iter().sum()
}

fn get_next(input: &Vec<i32>) -> i32 {
    if input.iter().all(|&x| x == 0) {
        return 0;
    }

    let mut out = Vec::new();
    for i in 1..input.len() {
        let diff = input[i] - input[i - 1];
        out.push(diff);
    }

    return out.last().unwrap() + get_next(&out);
}

fn main() {
    let input = read_input();
    let mut parsed = parse_input(&input);
    // Part 2
    parsed.iter_mut().for_each(|x| x.reverse());
    let result = solve(&parsed);
    println!("{}", result);
}
