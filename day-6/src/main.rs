use regex::Regex;
use std::io::stdin;
use std::str::FromStr;

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

fn parse_input(input: &Vec<String>) -> Vec<(i128, i128)> {
    let re = Regex::new(r"(\d+)").unwrap();

    let time_line = &input[0];
    let dist_line = &input[1];

    let f = |x: &str| i128::from_str(x).unwrap();

    let time_caps: Vec<i128> = re.find_iter(time_line).map(|x| f(x.as_str())).collect();
    let dist_caps: Vec<i128> = re.find_iter(dist_line).map(|x| f(x.as_str())).collect();

    let out = time_caps
        .iter()
        .zip(dist_caps.iter())
        .map(|(&x, &y)| (x, y))
        .collect();

    out
}

fn calc_result(parsed: &Vec<(i128, i128)>) -> i128 {
    let mut out = Vec::new();
    for (time, dist) in parsed {
        let mut speed = 0;
        let mut time_held = 0;
        let mut records_beat = 0;
        for _ in 0..*time {
            time_held += 1;
            speed += 1;

            if speed * (*time - time_held) > *dist {
                records_beat += 1;
            }
        }
        out.push(records_beat);
    }
    out.iter().product()
}

fn main() {
    let input = read_input();
    let parsed = parse_input(&input);
    let result = calc_result(&parsed);
    print!("{}", result)
}
