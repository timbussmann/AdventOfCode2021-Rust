use std::{fs, io::BufRead, collections::{HashSet, HashMap}};

struct Cave {
    name: String,
    adjacent: Vec<String>
}

pub fn puzzle1(){
    let input = fs::read_to_string("input-day12.txt").expect("error reading input file");

    let mut caves: HashMap<&str, HashSet<&str>> = HashMap::new();
    for line in input.lines(){
        let split: Vec<&str> = line.split('-').collect();
        let from = split[0];
        let to = split[1];

        if let Some(existing) = caves.get_mut(from) {
            existing.insert(to);
        } else {
            caves.insert(from, HashSet::from([to]));
        }

        // also register reverse route
        if let Some(exsting) = caves.get_mut(to) {
            exsting.insert(from);
        } else {
            caves.insert(to, HashSet::from([from]));
        }
    }
    //dbg!(&caves);

    let paths = walk_path(vec![String::from("start")], &caves);
    //dbg!(&paths);
    println!("found {} paths", paths.len());
}

pub fn puzzle2(){
    let input = fs::read_to_string("input-day12.txt").expect("error reading input file");

    let mut caves: HashMap<&str, HashSet<&str>> = HashMap::new();
    for line in input.lines(){
        let split: Vec<&str> = line.split('-').collect();
        let from = split[0];
        let to = split[1];

        if let Some(existing) = caves.get_mut(from) {
            existing.insert(to);
        } else {
            caves.insert(from, HashSet::from([to]));
        }

        // also register reverse route
        if let Some(exsting) = caves.get_mut(to) {
            exsting.insert(from);
        } else {
            caves.insert(to, HashSet::from([from]));
        }
    }
    //dbg!(&caves);

    let paths = walk_path2(vec![String::from("start")], &caves);
    println!("found {} paths", paths.len());
}

fn walk_path(path: Vec<String>, map: &HashMap<&str, HashSet<&str>>) -> Vec<Vec<String>> {
    let last = path.last().unwrap().as_str();
    
    if last == "end" {
        return vec![path];
    }

    if let Some(connections) = map.get(last){
        return connections.iter().flat_map(|next| {
            let mut next_path = path.clone();
    
            let first_char: char = next.chars().next().unwrap();
            if first_char.is_lowercase() && path.contains(&next.to_string()) {
                return vec![]; // don't go back
            }
    
            next_path.push(String::from(*next));
            walk_path(next_path, map)
        }).collect();
    }

     vec![]
}

fn walk_path2(path: Vec<String>, map: &HashMap<&str, HashSet<&str>>) -> Vec<Vec<String>> {
    let last = path.last().unwrap().as_str();
    
    if last == "end" {
        return vec![path];
    }

    if let Some(connections) = map.get(last){
        return connections.iter().flat_map(|next| {
            let mut next_path = path.clone();
    
            let first_char: char = next.chars().next().unwrap();
            if first_char.is_lowercase() && path.contains(&next.to_string()) {
                // we can visit a single small case twice, check if we already have a single small cave twice:
                if *next == "start" {
                    return vec![]; // don't go back
                }

                for visited in path.iter() {            
                    if path.iter().filter(|s| *s == visited && s.chars().next().unwrap().is_lowercase()).count() > 1 {
                        return vec![];
                    }
                }

                // we can do a second visit, fall through
            }
    
            next_path.push(String::from(*next));
            walk_path2(next_path, map)
        }).collect();
    }

     vec![]
}