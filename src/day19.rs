#![allow(unused_variables)]

use std::{fs, io::BufRead, collections::{HashSet, HashMap}, ops::{Sub, Add, Index, IndexMut, RangeBounds}, iter::TakeWhile, fmt::Debug};

type Coordinate = (isize, isize, isize);

pub fn puzzle1(){
    let input = fs::read_to_string("input-day19.txt").expect("error reading input file");
    let groups = input.split("\n\n");
    let groups: Vec<(u32, Vec<Coordinate>)> = groups.into_iter().map(|g| {
        let mut lines = g.lines();
        let group_no = lines.next().unwrap().strip_prefix("--- scanner ").unwrap().strip_suffix(" ---").unwrap();
        
        let mut coords: Vec<Coordinate> = Vec::new();
        for l in lines {
            let p: Vec<&str> = l.split(',').collect();
            coords.push((p[0].parse().unwrap(), p[1].parse().unwrap(), p[2].parse().unwrap()));
        }

        (group_no.parse().unwrap(), coords)
    }).collect();
    //dbg!(groups);

}

fn get_vectors(p: &Coordinate, ps: Vec<Coordinate>) -> Vec<Coordinate> {
    let mut vectors: Vec<Coordinate> = Vec::new();
    for p_b in ps.iter() {
        if p != p_b {
            vectors.push((p_b.0 - p.0, p_b.1 - p.1, p_b.2 - p.2));    
        }
    }

    vectors
}