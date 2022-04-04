use std::{fs, io::BufRead, collections::{HashSet, HashMap}, ops::{Sub, Add, Index, IndexMut, RangeBounds}, iter::TakeWhile, fmt::Debug};


pub fn puzzle1() {
    let input = fs::read_to_string("input-day25.txt").expect("error reading input file");
    let lines = input.lines();
    let mut map : Vec<Vec<char>> = lines.map(|l| l.chars().collect()).collect();

    let mut steps = 0;
    loop {
        steps += 1;
        let result = loop_once(&map);
        map = result.0;
        if result.1 == 0 {
            break;
        }
    }
    
    print_vec(&map);
    println!("Stop after {} iterations", steps);
}

fn loop_once(map: &Vec<Vec<char>>) -> (Vec<Vec<char>>, usize){

    let mut changes = 0;
    let mut new_map = map.clone();

    //TODO we don't really need the reverse?
    for (i,y) in map.iter().enumerate() {
        for x in (0..y.len()).rev(){
            if y[x] == '>' {
                let new_pos = (x + 1) % y.len();
                if y[new_pos] == '.' {
                    new_map[i][new_pos] = '>';
                    new_map[i][x] = '.';
                    changes += 1;
                }
            }
        }
    }

    let map = new_map;
    new_map = map.clone();
    for (y, row) in map.iter().enumerate() {
        for x in 0..row.len() {
            if row[x] == 'v' {
                let new_pos = (y + 1) % map.len();
                if map[new_pos][x] == '.' {
                    new_map[new_pos][x] = 'v';
                    new_map[y][x] = '.';
                    changes += 1;
                }
            }
        }
    }

    (new_map, changes)
}

fn print_vec(vec: &[Vec<char>]) {
    for l in vec {
        for c in l {
            print!("{}", c);
        }
        println!();
    }
}