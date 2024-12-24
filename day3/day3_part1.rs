use std::fs;
use regex::Regex;


fn solve(content: &str) -> i32 {
    let regex_finder = Regex::new(r"mul\((\d{1,3}),(\d{1,3})\)").expect("Could not parse regex");
    let mut answer: i32 = 0;

    for (full, [num1, num2]) in regex_finder.captures_iter(content).map(|m| m.extract()) {
        let a: i32 = num1.parse().expect("Could not parse numbers");
        let b: i32 = num2.parse().expect("Could not parse numbers");

        answer += a * b;
    }

    answer
}

fn main() {
    let content = fs::read_to_string("./input.txt").expect("Could not read file");
    let answer = solve(&content);

    println!("{answer}");
}

