use std::{fs, io::BufRead};

#[derive(Debug)]
struct Move {
    direction: Direction,
    amount: i32
}

#[derive(Debug)]
enum Direction {
    Forward,
    Up,
    Down
}

pub fn puzzle3() {
    let movements = load_movements();

    let (x, y) : (i32, i32) = movements.iter().fold((0,0), |(x, y), m| {
        match m.direction {
            Direction::Forward => (x + m.amount, y),
            Direction::Up => (x, y - m.amount), // up drecreases depth
            Direction::Down => (x, y + m.amount) // down increases depth
        }
    });

    println!("Solution 2.1 is {}", x * y)
}

pub fn puzzle4() {
    let movements = load_movements();

    let (x, y, _) : (i32, i32, i32) = movements.iter().fold((0,0,0), |(x, y, aim), m| {
        match m.direction {
            Direction::Forward => (x + m.amount, y + (aim * m.amount), aim),
            Direction::Up => (x, y, aim - m.amount),
            Direction::Down => (x, y, aim + m.amount)
        }
    });

    println!("Solution 2.2 is {}", x * y)
}

fn load_movements() -> Vec<Move> {
    let contents = fs::read_to_string("input-day2.txt").expect("error reading input file");
    let movements : Vec<Move> = contents
        .lines()
        .map(|l| l.split(' ').collect())
        .map(|m : Vec<&str>| Move { direction: get_direction(m[0]), amount: m[1].parse().expect("parse error")})
        .collect();
    movements
}

fn get_direction(input: &str) -> Direction {
    match input.to_lowercase().as_str() {
        "up" => Direction::Up,
        "down" => Direction::Down,
        "forward" => Direction::Forward,
        _ => panic!("direction unknown")
    }
}