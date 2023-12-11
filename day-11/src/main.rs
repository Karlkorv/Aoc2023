fn read_input() -> Vec<String> {
    let mut input = Vec::new();
    loop {
        let mut line = String::new();
        let bytes_read = std::io::stdin().read_line(&mut line).unwrap();
        if bytes_read == 0 {
            break;
        }
        input.push(line.trim().to_string());
    }
    input
}

// part one
// fn parse_input(input: Vec<String>) -> Vec<(usize, usize)> {
//     let mut galaxies: Vec<Vec<bool>> = input
//         .iter()
//         .map(|str| {
//             str.chars()
//                 .map(|c| match c {
//                     '.' => false,
//                     '#' => true,
//                     _ => panic!("Invalid input"),
//                 })
//                 .collect()
//         })
//         .collect();

//     let rows_to_expand: Vec<usize> = galaxies
//         .iter()
//         .enumerate()
//         .filter(|(_, row)| row.iter().all(|x| !x))
//         .map(|(i, _)| i)
//         .collect();

//     let mut cols_to_expand: Vec<usize> = Vec::new();

//     for i in 0..galaxies.len() {
//         let mut col_empty = true;
//         for j in 0..galaxies[i].len() {
//             col_empty &= !galaxies[j][i]; // inverterad indexering
//         }
//         if col_empty {
//             cols_to_expand.push(i);
//         }
//     }

//     println!("Expanding rows: {:?}", rows_to_expand);
//     println!("Expanding cols: {:?}", cols_to_expand);

//     // först och sista kolonnen/raden är inte tomma
//     for i in rows_to_expand.iter().rev() {
//         galaxies.insert(i + 1, vec![false; galaxies[*i].len()]);
//     }

//     for i in cols_to_expand.iter().rev() {
//         for row in &mut galaxies {
//             row.insert(i + 1, false);
//         }
//     }

//     galaxies
//         .iter()
//         .enumerate()
//         .flat_map(|(i, row)| {
//             row.iter()
//                 .enumerate()
//                 .filter(|(_, x)| **x)
//                 .map(move |(j, _)| (i, j))
//         })
//         .collect()
// }
fn parse_input(input: Vec<String>) -> Vec<(usize, usize)> {
    const ONE_MILLION_DOLLARS: usize = 999999;
    let galaxies: Vec<Vec<bool>> = input
        .iter()
        .map(|str| {
            str.chars()
                .map(|c| match c {
                    '.' => false,
                    '#' => true,
                    _ => panic!("Invalid input"),
                })
                .collect()
        })
        .collect();

    let rows_to_expand: Vec<usize> = galaxies
        .iter()
        .enumerate()
        .filter(|(_, row)| row.iter().all(|x| !x))
        .map(|(i, _)| i)
        .collect();

    let mut cols_to_expand: Vec<usize> = Vec::new();

    for i in 0..galaxies.len() {
        let mut col_empty = true;
        for j in 0..galaxies[i].len() {
            col_empty &= !galaxies[j][i]; // inverterad indexering
        }
        if col_empty {
            cols_to_expand.push(i);
        }
    }

    println!("Expanding rows: {:?}", rows_to_expand);
    println!("Expanding cols: {:?}", cols_to_expand);

    let stars: Vec<(usize, usize)> = galaxies
        .iter()
        .enumerate()
        .flat_map(|(i, row)| {
            row.iter()
                .enumerate()
                .filter(|(_, x)| **x)
                .map(move |(j, _)| (i, j))
        })
        .collect();

    println!("Star positions before expansion: {:?}", stars);

    let out = galaxies
        .iter()
        .enumerate()
        .flat_map(|(i, row)| {
            row.iter()
                .enumerate()
                .filter(|(_, x)| **x)
                .map(move |(j, _)| (i, j))
        })
        .map(|(i, j)| {
            (
                rows_to_expand.iter().filter(|x| **x < i).count() * ONE_MILLION_DOLLARS + i,
                cols_to_expand.iter().filter(|x| **x < j).count() * ONE_MILLION_DOLLARS + j,
            )
        })
        .collect();
    println!("Star positions after expansion: {:?}", out);
    out
}

fn solve(input: Vec<(usize, usize)>) -> isize {
    let mut distances = vec![];
    for (i, (x, y)) in input.iter().enumerate() {
        for (x1, y1) in input.iter().skip(i) {
            if x1 == x && y1 == y {
                continue;
            }
            let x_dist = (*x as isize - *x1 as isize).abs();
            let y_dist = (*y as isize - *y1 as isize).abs();
            distances.push(x_dist + y_dist);
        }
    }

    distances.iter().sum()
}

fn main() {
    let input = read_input();
    let parsed = parse_input(input);
    let result = solve(parsed);
    println!("{}", result);
}
