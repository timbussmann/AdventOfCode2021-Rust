use std::{fs, io::BufRead};

pub fn puzzle1() {
    let input = fs::read_to_string("input-day7.txt").expect("error reading input file");
    let positions : Vec<i32> = input.split(',').map(|s| s.parse().unwrap()).collect();

    let min = positions.iter().min().unwrap();
    let max = positions.iter().max().unwrap();

    let mut smallest = (i32::MAX, 0);
    for i in *min..*max {
        let fuel_cost : i32 = positions.iter().map(|x| (x - i).abs()).sum();
        if fuel_cost < smallest.0 {
            smallest = (fuel_cost, i);
        }
    }

    println!("Least fuel cost on position {} with cost {}", smallest.1, smallest.0);
}

pub fn puzzle2() {
    let input = fs::read_to_string("input-day7.txt").expect("error reading input file");
    let positions : Vec<i32> = input.split(',').map(|s| s.parse().unwrap()).collect();

    let min = positions.iter().min().unwrap();
    let max = positions.iter().max().unwrap();

    let mut smallest = (i32::MAX, 0);
    for i in *min..*max {
        let fuel_cost : i32 = positions.iter().map(|x| {
            let n = (x - i).abs();
            (n * (n + 1))/2 // triangular number
        }).sum();
        if fuel_cost < smallest.0 {
            smallest = (fuel_cost, i);
        }
    }

    println!("Least fuel cost on position {} with cost {}", smallest.1, smallest.0);
}