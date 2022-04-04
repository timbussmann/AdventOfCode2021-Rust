use std::{fs, io::BufRead};

pub fn puzzle1(){
    let input = fs::read_to_string("input-day10.txt").expect("error reading input file");
    let lines = input.lines();

    let mut invalid_chars = Vec::new();
    for line in lines {
        if let Some(invalid_char) = find_invalid_character(line) {
            invalid_chars.push(invalid_char);
        }      
    }

    let score : u32 = invalid_chars.iter().map(|c| {
        match c {
            ')' => 3,
            ']' => 57,
            '}' => 1197,
            '>' => 25137,
            _ => panic!("unexpected character in score calculation")
        }
    }).sum();

    println!("Result is {}", score);
}

fn find_invalid_character(line: &str) -> Option<char> {
    let mut stack = Vec::new();
    for char in line.chars() {
        match char {
            //TODO complete allows brackets!
            '(' | '[' | '{' | '<' => stack.push(char),
            ')' | ']' | '}' | '>' => {
                let last = stack.pop().expect("missing opening character");
                match last {
                    '(' if char == ')' => (),
                    '[' if char == ']' => (),
                    '{' if char == '}' => (),
                    '<' if char == '>' => (),
                    _ => return Some(char)
                }
            }
            _ => panic!("unexpected character")
        }
    }

    None
}

pub fn puzzle2() {
    let input = fs::read_to_string("input-day10.txt").expect("error reading input file");
    let lines = input.lines();

    let mut incomplete_lines = Vec::new();
    for line in lines {
        if let Some(incomplete_line) = find_incomplete_line(line) {
            incomplete_lines.push(incomplete_line);
        }      
    }

    dbg!(&incomplete_lines);

    let mut scores: Vec<u128> = incomplete_lines.iter().map(|l| l.iter().fold(0, |x, char| x * 5 + points(char))).collect();
    scores.sort_unstable();
    let len = scores.len() / 2;
    println!("middle index for {} is {}", scores.len(), len);
    let score = scores[len];
    println!("Result is {}", score);
}

fn find_incomplete_line(line: &str) -> Option<Vec<char>>{
    let mut stack = Vec::new();
    for char in line.chars() {
        match char {
            //TODO complete allows brackets!
            '(' | '[' | '{' | '<' => stack.push(char),
            ')' | ']' | '}' | '>' => {
                let last = stack.pop().expect("missing opening character");
                match last {
                    '(' if char == ')' => (),
                    '[' if char == ']' => (),
                    '{' if char == '}' => (),
                    '<' if char == '>' => (),
                    _ => return None
                }
            }
            _ => panic!("unexpected character")
        }
    }

    stack.reverse();
    Some(stack)
}

fn points(char: &char) -> u128 {
    match char {
        '(' => 1,
        '[' => 2,
        '{' => 3,
        '<' => 4,
        _ => panic!("unexpected character")
    }
}