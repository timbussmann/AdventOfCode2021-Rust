

use std::{fs, io::BufRead, collections::HashMap};


pub fn puzzle1() {
    let input = fs::read_to_string("input-day6.txt").expect("error reading input file");
    let mut init = parse_input(&input);
    
    loop_for(&mut init, 80);

    println!("Total lanternfish at day 80: {}", init.len())
}


pub fn puzzle2() {
    let input = fs::read_to_string("input-day6.txt").expect("error reading input file");
    let init = parse_input(&input);
    
    let mut groups = group(&init); // no easy-to-use group-by available, let's build our own shitty implementation
    for _ in 0..256 {
        groups = loop_group(groups);
    }

    println!("Total lanternfish at day 256: {}", groups.iter().sum::<usize>());
    
}

fn group(school:&[usize]) -> [usize; 9] {
    let mut result = [0; 9];
    for fish in school {
        result[*fish] += 1;
    }

    result
}

// rust doesn't seem to have a nice group-by like C#
fn loop_group(group:[usize; 9]) -> [usize; 9]{
    let mut result = [0;9];
    let new = group[0];
    result[..(9 - 1)].clone_from_slice(&group[1..9]); // "shift" array to left
    result[6] += new;
    result[8] += new;

    result
}

fn parse_input(input:&str) -> Vec<usize> {
    input.split(',').map(|n| n.parse::<usize>().unwrap()).collect()
}

fn loop_for(school:&mut Vec<usize>, days:usize){
    for _ in 0..days {
        loop_day(school);
    }
}

fn loop_day(school:&mut Vec<usize>) {
    let mut new = 0;
    for fish in school.iter_mut() {
        if *fish == 0 {
            *fish = 6;
            new += 1;
        } else {
            *fish -= 1;
        }
    }

    for _ in 0..new {
        school.push(8);
    }
}