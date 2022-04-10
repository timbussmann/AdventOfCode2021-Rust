use std::{fs, io::BufRead, collections::HashSet, panic};

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

pub fn puzzle2(){
    let input = fs::read_to_string("input-day8.txt").expect("error reading input file");
    let display_values : Vec<(Vec<&str>, Vec<&str>)> = input
        .lines()
        .map(|l| l.split('|'))
        .map(|mut splits| (splits.next().unwrap().split_whitespace().collect(), splits.next().unwrap().split_whitespace().collect()))
        .collect();

    let mappings : Vec<(&Vec<&str>, Vec<HashSet<char>>)> = display_values.iter().map(|(a, b)| (b, get_mapping(a))).collect();
    let digits : Vec<Vec<usize>> = mappings.iter().map(|(input, map)| input.iter().map(|c| segments_to_digit(&HashSet::from_iter(c.chars()), map)).collect()).collect();

    let row_sums : Vec<u32> = digits.iter().map(|digits| combine_digits(digits)).collect();
    let total_sum : u32 = row_sums.iter().sum();
    dbg!(total_sum);
}

fn get_mapping(signals: &Vec<&str>) -> Vec<HashSet<char>>{
    let mut result: Vec<HashSet<char>> = vec![HashSet::new(); 10];
    let signals_with_length : Vec<(usize, HashSet<char>)> = signals.iter().map(|s| (s.len(), s.chars().collect())).collect();

    // handle the unique cases first
    result[1] = signals_with_length.iter().filter(|(l, _)| *l == 2).map(|(_, c)| c).next().unwrap().clone();
    result[4] = signals_with_length.iter().filter(|(l, _)| *l == 4).map(|(_, c)| c).next().unwrap().clone();
    result[7] = signals_with_length.iter().filter(|(l, _)| *l == 3).map(|(_, c)| c).next().unwrap().clone();
    result[8] = signals_with_length.iter().filter(|(l, _)| *l == 7).map(|(_, c)| c).next().unwrap().clone();
    
    // remaining digits using 6 segments
    result[6] = signals_with_length.iter().filter(|(l, _)| *l == 6).filter(|(_, chars)| overlaps(&result[1], chars) == 1).map(|(_, c)| c).next().unwrap().clone();
    result[9] = signals_with_length.iter().filter(|(l, _)| *l == 6).filter(|(_, chars)| overlaps(&result[4], chars) == 4).map(|(_, c)| c).next().unwrap().clone();
    result[0] = signals_with_length.iter().filter(|(l, _)| *l == 6).filter(|(_, chars)| overlaps(&result[1], chars) == 2 && overlaps(&result[4], chars) == 3).map(|(_, c)| c).next().unwrap().clone();

    // remaining digits using 5 segments
    result[3] = signals_with_length.iter().filter(|(l, _)| *l == 5).filter(|(_, chars)| overlaps(&result[1], chars) == 2).map(|(_, c)| c).next().unwrap().clone();
    result[2] = signals_with_length.iter().filter(|(l, _)| *l == 5).filter(|(_, chars)| overlaps(&result[1], chars) == 1 && overlaps(&result[4], chars) == 2).map(|(_, c)| c).next().unwrap().clone();
    result[5] = signals_with_length.iter().filter(|(l, _)| *l == 5).filter(|(_, chars)| overlaps(&result[1], chars) == 1 && overlaps(&result[4], chars) == 3).map(|(_, c)| c).next().unwrap().clone();

    result
}

fn overlaps(a: &HashSet<char>, b: &HashSet<char>) -> usize {
    a.intersection(b).count()
}

fn segments_to_digit(seg: &HashSet<char>, mapping: &Vec<HashSet<char>>) -> usize {
    for (i, s) in mapping.iter().enumerate() {
        if s.len() == seg.len() && s.intersection(seg).count() == seg.len() {
            return i;
        }
    }
    
    dbg!(seg);
    dbg!(mapping);
    panic!("no match found")
}

fn combine_digits(digits: &Vec<usize>) -> u32{
    digits.iter().fold(0, |acc, elem| acc * 10 + (*elem as u32))
}