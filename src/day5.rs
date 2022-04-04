use std::ops::Index;
use std::vec;
use std::{fs, io::BufRead};
use std::str::{self, Lines};
use std::cmp;

type Line = (Coordinate, Coordinate);
type Coordinate = (usize, usize);
pub fn puzzle1() {
    let input = fs::read_to_string("input-day5.txt").expect("error reading input file");
    let data = input.lines();

    let mut coords : Vec<Line> = data
    .map(|l| l.split("->")
        .map(|x| {
            let coords = x.trim().split(',').collect::<Vec<&str>>();
            ((*coords[0]).parse().unwrap(), (*coords[1]).parse().unwrap())
        }).collect::<Vec<Coordinate>>())
    .map(|l| (l[0], l[1]))
    .collect();
    println!("{:?}", coords);

    let max_x : usize = coords.iter().flat_map(|cp| vec![cp.0.0, cp.1.0]).max().unwrap();
    let max_y : usize = coords.iter().flat_map(|cp| vec![cp.0.1, cp.1.1]).max().unwrap();
    println!("max values are x: {}, y: {}", max_x, max_y);

    coords.retain(|((x1, y1), (x2, y2))| x1 == x2 || y1 == y2);
    let mut map = vec![vec![0; max_x + 1]; max_y + 1];
    for ((x1, y1), (x2, y2)) in coords {
        if x1 == x2 {
            // mark along y axis
            for y in cmp::min(y1, y2)..(cmp::max(y1, y2) + 1) { // rust range are exclusive on the upper end!
                map[y][x1] += 1;
            }
        } else {
            // mark along x axis
            for x in cmp::min(x1, x2)..(cmp::max(x1, x2) + 1) {
                map[y1][x] += 1;
            }
        }
    }
    //println!("{:?}", map);

    let overlaps : usize = map.iter().map(|row| row.iter().filter(|x| **x >= 2).count()).sum();
    println!("Result for puzzle is: {}", overlaps);
}

pub fn puzzle2(){
    let input = fs::read_to_string("input-day5.txt").expect("error reading input file");
    let data = input.lines();

    let coords : Vec<Line> = data
    .map(|l| l.split("->")
        .map(|x| {
            let coords = x.trim().split(',').collect::<Vec<&str>>();
            ((*coords[0]).parse().unwrap(), (*coords[1]).parse().unwrap())
        }).collect::<Vec<Coordinate>>())
    .map(|l| (l[0], l[1]))
    .collect();
    //println!("{:?}", coords);

    let max_x : usize = coords.iter().flat_map(|cp| vec![cp.0.0, cp.1.0]).max().unwrap();
    let max_y : usize = coords.iter().flat_map(|cp| vec![cp.0.1, cp.1.1]).max().unwrap();
    println!("max values are x: {}, y: {}", max_x, max_y);

    let mut map = vec![vec![0; max_x + 1]; max_y + 1];
    for ((x1, y1), (x2, y2)) in coords {
        
        if x1 == x2 {
            // mark along y axis
            for y in cmp::min(y1, y2)..(cmp::max(y1, y2) + 1) { // rust range are exclusive on the upper end!
                map[y][x1] += 1;
            }
        } else if y1 == y2 {
            // mark along x axis
            for x in cmp::min(x1, x2)..(cmp::max(x1, x2) + 1) {
                map[y1][x] += 1;
            }
        } else {
            let y = (y1..=y2).collect::<Vec<usize>>();
            let y = if y.is_empty() { (y2..=y1).rev().collect::<Vec<usize>>() } else { y };
            let x = (x1..=x2).collect::<Vec<usize>>();
            let x = if x.is_empty() { (x2..=x1).rev().collect::<Vec<usize>>() } else { x };
            println!("{:?}", y);
            println!("{:?}", x);
            let coords : Vec<(&usize, &usize)> = x.iter().zip(y.iter()).collect();

            println!("line from ({},{}) to ({},{}) via {:?}", x1, y1, x2, y2, coords);
            for (xx, yy) in coords {
                map[*yy][*xx] += 1;
            }
        }
    }
    //println!("{:?}", map);

    let overlaps : usize = map.iter().map(|row| row.iter().filter(|x| **x >= 2).count()).sum();
    println!("Result for puzzle is: {}", overlaps);
}