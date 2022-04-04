use std::{fs, io::BufRead};

pub fn puzzle1(){
    let input = fs::read_to_string("input-day11.txt").expect("error reading input file");
    let mut data: Vec<Vec<(u32, bool)>> = input
        .lines()
        .map(|l| l.chars()
            .map(|c| (c.to_digit(10).unwrap(), false))
            .collect())
        .collect();

    let result = (0..100).fold(0, |acc, _i | acc + step(&mut data));
    println!("Result is {}", result);
}

pub fn puzzle2() {
    let input = fs::read_to_string("input-day11.txt").expect("error reading input file");
    let mut data: Vec<Vec<(u32, bool)>> = input
        .lines()
        .map(|l| l.chars()
            .map(|c| (c.to_digit(10).unwrap(), false))
            .collect())
        .collect();

    for i in 1.. {
        step(&mut data);
        let sum: u32 = data.iter().map(|line| line.iter().map(|pos| pos.0).sum::<u32>()).sum();
        if sum == 0 {
            println!("synced at step {}", i);
            return;
        }
    }
}

fn step(data: &mut Vec<Vec<(u32, bool)>>) -> usize {
    // increment all
    for line in data.iter_mut() {
        for (point, _) in line.iter_mut() {
            *point += 1;
        }
    }

    // flash all with full energy
    let mut flashes : usize = 0;
    for y in 0..data.len() {
        for x in 0..data[y].len() {
            flashes += flash(x, y, data);
        }
    }

    // reset flash tracker
    for line in data.iter_mut() {
        for (value, flashed) in line.iter_mut() {
            if *value > 9 {
                *value = 0;
            }
            *flashed = false;

        }
    }

    //dbg!(&data);
    dbg!(flashes);
    flashes
}

fn flash(x: usize, y: usize, data: &mut Vec<Vec<(u32, bool)>>) -> usize {
    let (a, b) = &mut data[y][x];
    if *a > 9 && !*b {
        *b = true;
        let mut flashes = 1;
        for dy in [-1, 0, 1] {
            for dx in [-1, 0, 1] {

                if dy == 0 && dx == 0 {
                    continue; //current pos
                }

                let y_pos : i32  = i32::try_from(y).unwrap() + dy;
                let x_pos : i32 = i32::try_from(x).unwrap() + dx;

                if y_pos < 0 || y_pos >= data.len().try_into().unwrap() {
                    continue;
                }
                if x_pos < 0 || x_pos >= data[0].len().try_into().unwrap() {
                    continue;
                }

                
                let (neighbor_value, _) = &mut data[y_pos as usize][x_pos as usize];
                *neighbor_value += 1;
                flashes += flash(x_pos as usize, y_pos as usize, data);
            }
        }
        return flashes;
    }

    0
}