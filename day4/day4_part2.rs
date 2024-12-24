use std::fs;

fn solve(data: &[&str]) -> i32 {
    (0..data.len() as isize)
        .flat_map(|x| (0..data[0].len() as isize).map(move |y| (x, y)))
        .map(|(x, y)| {
            [
                [(x + 1, y - 1), (x, y), (x - 1, y + 1)], // NE
                [(x + 1, y + 1), (x, y), (x - 1, y - 1)], // SE
            ]
        })
        .filter(|coord| {
            let iter = coord.iter().filter(|row| {
                let row_collect = row.map(|(x, y)| data.get(x as usize).and_then(|inp| inp.as_bytes().get(y as usize).copied()).unwrap_or_default());
                &row_collect == b"MAS" || &row_collect == b"SAM"
            }).count();

            iter == 2
        })
        .count() as i32
}

fn main() {
    let data = fs::read_to_string("./input.txt").expect("Could not read the file to string");
    let data_lines: Vec<_> = data.lines().collect();

    println!("{}", solve(&data_lines));
}

