use std::{fs, io::BufRead};

pub fn puzzle1(){
    let input = fs::read_to_string("input-day9.txt").expect("error reading input file");
    let rows : Vec<Vec<u32>> = input.lines().map(|l| l.chars()
                                    .map(|c| c.to_digit(10).unwrap()).collect()).collect();

    let mut lowest: Vec<u32> = Vec::new();
    for y in 0..rows.len() {
        let row = &rows[y];
        for x in 0..row.len() {
            let nearest = [get_nearest_x(x, row), get_nearest_y(x, y, &rows)].concat();
            let value = rows[y][x];
            if test(value, nearest) {
                lowest.push(value);
            }
        }
    }

    let result: u32 = lowest.iter().sum::<u32>() + lowest.len() as u32;
    println!("result is {}", result);
}

fn get_nearest_x(x: usize, map: &[u32]) -> Vec<u32> {
    let max_x = map.len() - 1;
    match x {
        0 => vec![map[1]],
        z if z == max_x => vec![map[max_x - 1]],
        z if z < max_x =>  vec![map[x - 1], map[x + 1]],
        _ => panic!("out of bound")
    }
}

fn get_nearest_y(x: usize, y: usize, map: &[Vec<u32>]) -> Vec<u32> {
    let max_y = map.len() - 1;
    match y {
        0 => vec![map[1][x]],
        z if z < max_y => vec![map[y - 1][x], map[y + 1][x]],
        z if z == max_y => vec![map[max_y - 1][x]],
        _ => panic!("out of bound")
    }
}

fn test (x: u32, xs: Vec<u32>) -> bool {
    x < *(xs.iter().min().unwrap())
}