use std::{fs, io::BufRead};

pub fn puzzle1(){
    let input = fs::read_to_string("input-day8.txt").expect("error reading input file");
    let outputs : Vec<usize> = input
        .lines()
        .flat_map(|l| l.split('|').last().unwrap().split_whitespace()
            .map(|output| (*output).len()).collect::<Vec<usize>>()).collect();

    // unique segments:
    // 1 = 2 segments
    // 4 = 4 segments
    // 7 = 3 segments
    // 8 = 7 segments
    let unique_segment_lengths : Vec<usize> = vec![2, 3, 4, 7];
    let result = outputs.iter().filter(|d| unique_segment_lengths.contains(*d)).count();
    println!("Result is {}", result);
}