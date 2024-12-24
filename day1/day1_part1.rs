use std::{io::{self, BufRead, BufReader}, fs};

fn solve(mut input1: Vec<i32>, mut input2: Vec<i32>) {
    let mut answer = 0; // Let Rust infer the type

    input1.sort();
    input2.sort();

    answer = input1.iter().zip(&input2).map(|(a, b)| (a - b).abs()).sum();

    println!("The total absolute difference is: {answer}");
}

fn parse_line(line: &str) -> (i32, i32) {
    let numbers: Vec<i32> = line
        .trim()
        .split_whitespace()
        .map(|x| x.parse::<i32>().expect("Parsing error: expected an integer"))
        .collect();

    if numbers.len() != 2 {
        panic!("Each line must contain exactly two integers");
    }

    (numbers[0], numbers[1])
}

fn main() {
    let mut input1 = Vec::new();
    let mut input2 = Vec::new();

    let input_file = fs::File::open("./input.txt").expect("Could not open the input file");
    let reader = BufReader::new(input_file);

    for line in reader.lines() {
        match line {
            Ok(content) => {
                let (val1, val2) = parse_line(&content);
                input1.push(val1);
                input2.push(val2);
            }
            Err(e) => eprintln!("Error reading line: {e}"),
        }
    }

    solve(input1, input2);
}

