use std::{fs, io::BufRead, collections::{HashSet, HashMap}, ops::{Sub, Add, Index, IndexMut, RangeBounds}, iter::TakeWhile, fmt::Debug};

pub fn puzzle1() {
    let input = fs::read_to_string("input-day20.txt").expect("error reading input file");
    let mut lines = input.lines();

    let algorithm: Vec<char> = lines.next().unwrap().chars().collect();
    //dbg!(algorithm);

    lines.next(); // empty line
    let start: Vec<Vec<char>> = lines.map(|l| l.chars().collect()).collect();
    //dbg!(start);

    // create an outer border of 2 with the default dark pixel so that we can easily grab the neighbors without further check for the current image + 1 (as it grows outwards)
    // in addition, the outer border represents the "infinity" dark pixels. Given they chan also change(!) we need to update those as well.
    let mut new = vec![vec!['.'; start[0].len() + 4]; start.len() + 4];
    for y in 0..start.len() {
        for x in 0..start[0].len() {
            new[y + 2][x + 2] = start[y][x];
        }
    }

    //print_vec(&new);
    let z1 = zoom(new, &algorithm); 
    //print_vec(&z1);
    let z2 = zoom(z1, &algorithm);
    print_vec(&z2);

    let active: usize = z2.iter().map(|l| l.iter().fold(0, |acc, c| acc + (if *c == '#' { 1 } else { 0 }))).sum();
    println!("{} active pixels", active);
}

pub fn puzzle2() {
    let input = fs::read_to_string("input-day20.txt").expect("error reading input file");
    let mut lines = input.lines();

    let algorithm: Vec<char> = lines.next().unwrap().chars().collect();
    //dbg!(algorithm);

    lines.next(); // empty line
    let start: Vec<Vec<char>> = lines.map(|l| l.chars().collect()).collect();
    //dbg!(start);

    let mut new = vec![vec!['.'; start[0].len() + 4]; start.len() + 4];
    for y in 0..start.len() {
        for x in 0..start[0].len() {
            new[y + 2][x + 2] = start[y][x];
        }
    }

    let mut current = new;
    for _ in 0..50 {
        current = zoom(current, &algorithm);
    }
    print_vec(&current);

    let active: usize = current.iter().map(|l| l.iter().fold(0, |acc, c| acc + (if *c == '#' { 1 } else { 0 }))).sum();
    println!("{} active pixels", active);
}

fn zoom(image: Vec<Vec<char>>, algorithm: &Vec<char>) -> Vec<Vec<char>>{
    let mut new2 = vec![vec!['X'; image[0].len() + 2]; image.len() + 2];
    for y in 1..(image.len() - 1) {
        for x in 1..(image[0].len() - 1) {
            let group = get_group_value(x, y, &image);
            let group = group.replace('#', "1").replace(".", "0");
            let digit = usize::from_str_radix(&group, 2).unwrap();
            let new_value = algorithm[digit];
            new2[y + 1][x + 1] = new_value;
        }
    }
    // update "infinite" border. Since the border is 2 indexes wide, we know it only contains "dark" (or flipped) pixels
    let key = image[0][0];
    let key = [key; 9];
    let key = String::from_iter(key).replace('#', "1").replace(".", "0");
    let index = usize::from_str_radix(key.as_str(), 2).unwrap();
    let new_value = algorithm[index];
    for y in &[0, 1, (new2.len() - 2), (new2.len() - 1)] {
        new2[*y] = vec![new_value; new2.len()];
    }

    for x in &[0, 1, (new2[0].len() - 2), (new2[0].len() - 1)] {
        for y in 0..new2.len() {
            new2[y][*x] = new_value;
        }
    }

    new2
}

fn get_group_value(x: usize, y: usize, image: &Vec<Vec<char>>) -> String {
    let mut result = String::new();
    for y in (y-1)..=(y + 1) {
        for x in (x-1)..=(x+1) {
            result.push(image[y][x]);
        }
    }
    result
}

fn print_vec(vec: &[Vec<char>]) {
    for l in vec {
        for c in l {
            print!("{}", c);
        }
        println!();
    }
}