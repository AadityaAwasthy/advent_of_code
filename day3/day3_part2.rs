use std::fs;
use regex::Regex;


fn parse_input(content: &str) -> i32 {
    let regex_finder = Regex::new(r"mul\((\d{1,3}),(\d{1,3})\)").expect("Could not parse regex");
    let mut answer: i32 = 0;

    for (full, [num1, num2]) in regex_finder.captures_iter(content).map(|m| m.extract()) {
        let a: i32 = num1.parse().expect("Could not parse numbers");
        let b: i32 = num2.parse().expect("Could not parse numbers");

        answer += a * b;
    }

    answer
}

fn solve(content: &str) -> i32 {
    let mut curr_offset: usize = 0;
    let dont_finder = Regex::new(r"don't\(\)").expect("Could not parse regex");
    let do_finder = Regex::new(r"do\(\)").expect("Could not parse regex");
    let mut answer: i32 = 0;

    loop {
        if let Some(found_dont) =  dont_finder.find(&content[curr_offset..]) {
            let until = found_dont.start();
            answer += parse_input(&content[curr_offset..curr_offset + until]);

            curr_offset += found_dont.end();
        } else {
            answer += parse_input(&content[curr_offset..]);
            break;
        }

        if let Some(found_do) = do_finder.find(&content[curr_offset..]) {
            curr_offset += found_do.end();
        } else {
            break;
        }

        if curr_offset >= content.len() {
            break;
        }
    }

    return answer;
}

fn main() {
    let content = fs::read_to_string("./input.txt").expect("Could not read file");
    let answer = solve(&content);

    println!("{answer}");
}

