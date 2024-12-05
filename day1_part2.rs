use std::{io::{self, BufRead, BufReader}, fs};

fn solve(input1: &mut Vec<i32>, input2: &mut Vec<i32>) {
    let mut hash = std::collections::HashMap::new();

    for num in input2 {
        hash.entry(num).and_modify(|current| *current += 1).or_insert(1);
    }

    let mut answer: i32 = 0;

    for num in input1 {
        match hash.get(num) {
            Some(freq) => {
                answer += (*num) * freq;
            },
            None => {}
        }
    }

    println!("{answer}");
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

    solve(&mut input1, &mut input2);
}

