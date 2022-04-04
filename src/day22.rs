use std::{fs, io::BufRead, collections::{HashSet, HashMap}, ops::{Sub, Add, Index, IndexMut, RangeBounds}, iter::TakeWhile, fmt::Debug};

#[derive(Debug)]
struct Instruction {
    toggle: bool,
    x: (isize, isize),
    y: (isize, isize),
    z: (isize, isize),
}

pub fn puzzle1() {
    let input = fs::read_to_string("input-day22.txt").expect("error reading input file");
    let lines = input.lines();

    let steps: Vec<Instruction> = lines.map(|l| {
        let toggle = l.starts_with("on");
        let trimmed = l.trim_start_matches("on ").trim_start_matches("off ");
        let splitted: Vec<&str> = trimmed.split(',').collect();
        let xc: Vec<&str> = splitted[0].trim_start_matches("x=").split("..").collect();
        let yc: Vec<&str> = splitted[1].trim_start_matches("y=").split("..").collect();
        let zc: Vec<&str> = splitted[2].trim_start_matches("z=").split("..").collect();
        
        Instruction {
            toggle,
            x: (xc[0].parse().unwrap(), xc[1].parse().unwrap()),
            y: (yc[0].parse().unwrap(), yc[1].parse().unwrap()),
            z: (zc[0].parse().unwrap(), zc[1].parse().unwrap()),
        }
    }).collect();

    let mut cube = vec![vec![vec![false; 101]; 101]; 101];
    for step in steps {
        for y in (std::cmp::max(-50, std::cmp::min(step.y.0, step.y.0)))..=(std::cmp::min(50, std::cmp::max(step.y.0, step.y.1)))
        {
            for x in (std::cmp::max(-50, std::cmp::min(step.x.0, step.x.0)))..=(std::cmp::min(50, std::cmp::max(step.x.0, step.x.1)))
            {
                for z in (std::cmp::max(-50, std::cmp::min(step.z.0, step.z.0)))..=(std::cmp::min(50, std::cmp::max(step.z.0, step.z.1)))
                {
                    let yt: usize = (y + 50).try_into().unwrap();
                    let xt: usize = (x + 50).try_into().unwrap();
                    let zt: usize = (z + 50).try_into().unwrap();
                    cube[yt][xt][zt] = step.toggle;
                }
            }
        }
    }

    let mut counter = 0;
    for y in cube {
        for x in y {
            for z in x {
                if z {
                    counter += 1;
                }
            }
        }
    }

    println!("counter is {}", counter);
}

pub fn puzzle2() {
    let input = fs::read_to_string("input-day22.txt").expect("error reading input file");
    let lines = input.lines();

    let steps: Vec<Instruction> = lines.map(|l| {
        let toggle = l.starts_with("on");
        let trimmed = l.trim_start_matches("on ").trim_start_matches("off ");
        let splitted: Vec<&str> = trimmed.split(',').collect();
        let xc: Vec<&str> = splitted[0].trim_start_matches("x=").split("..").collect();
        let yc: Vec<&str> = splitted[1].trim_start_matches("y=").split("..").collect();
        let zc: Vec<&str> = splitted[2].trim_start_matches("z=").split("..").collect();
        
        Instruction {
            toggle,
            x: (xc[0].parse().unwrap(), xc[1].parse().unwrap()),
            y: (yc[0].parse().unwrap(), yc[1].parse().unwrap()),
            z: (zc[0].parse().unwrap(), zc[1].parse().unwrap()),
        }
    }).collect();

    let mut cube: HashSet<(isize, isize, isize)> = HashSet::new();

    for (i, step) in steps.iter().enumerate() {
        for y in (std::cmp::min(step.y.0, step.y.0))..=(std::cmp::max(step.y.0, step.y.1))
        {
            for x in (std::cmp::min(step.x.0, step.x.0))..=(std::cmp::max(step.x.0, step.x.1))
            {
                for z in (std::cmp::min(step.z.0, step.z.0))..=(std::cmp::max(step.z.0, step.z.1))
                {
                    if step.toggle {
                        cube.insert((y, x, z));
                    } else {
                        cube.remove(&(y, x, z));
                    }
                }
            }
        }
        println!("step {} complete. Collection size {}", i, cube.len());
    }

    //let mut counter = cube.values().filter(|p| **p).count();
    let counter = cube.len();

    println!("counter is {}", counter);
}