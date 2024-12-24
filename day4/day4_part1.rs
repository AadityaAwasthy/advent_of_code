use std::fs;

fn solve(data: &[&str]) -> i32 {
    let mut word = [0; 4];
    (0..data.len() as isize)
        .flat_map(|x| (0..data[0].len() as isize).map(move |y| (x, y)))
        .flat_map(|(x, y)| {
            [
                [(x, y), (x + 1, y - 1), (x + 2, y - 2), (x + 3, y - 3)], // NE
                [(x, y), (x + 1, y), (x + 2, y), (x + 3, y)],             // E
                [(x, y), (x + 1, y + 1), (x + 2, y + 2), (x + 3, y + 3)], // SE
                [(x, y), (x, y + 1), (x, y + 2), (x, y + 3)],             // S
            ]
        })
        .filter(|coord| {
            let mut iter = coord.iter().map(|(x, y)| {
                data.get(*x as usize).and_then(|row| row.as_bytes().get(*y as usize).copied()).unwrap_or_default()
            });
            word.fill_with(|| iter.next().unwrap_or_default());
            &word == b"XMAS" || &word == b"SAMX"
        })
        .count() as i32
}

fn main() {
    let data = fs::read_to_string("./input.txt").expect("Could not read the file to string");
    let data_lines: Vec<_> = data.lines().collect();

    println!("{}", solve(&data_lines));
}

