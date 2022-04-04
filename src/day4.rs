use std::ops::Index;
use std::{fs, io::BufRead};
use std::str::{self, Lines};

type Block = Vec<Vec<i32>>;

pub fn puzzle1(){
    let data = fs::read_to_string("input-day4.txt").expect("error reading input file");
    let mut lines = data.lines();
    let numbers : Vec<i32> = lines.next().unwrap().split(',').map(|n| n.parse::<i32>().unwrap()).collect();
    println!("Drawn numbers are: {:?}", numbers);

    let blocks = get_blocks(lines);
    //println!("blocks: {:?}", blocks);

    let mut block_map : Vec<(&Block, Vec<Vec<bool>>)> = blocks.iter().map(|b| (b, b.iter().map(|y| y.iter().map(|_| false).collect()).collect())).collect();

    for n in numbers {
        // mark drawn number
        for (block, map) in block_map.iter_mut(){
            mark_number(n, block, map);
        }

        if let Some((winner_block, winner_map)) = find_winner(&block_map) {
            // calculate result for winning block
            println!("found winner drawing {}: {:?}, map: {:?}",n, winner_block, winner_map);
            let current_number = n;
            let mut unmarked_sum = 0;
            for y in 0..winner_map.len() {
                for x in 0..winner_map[0].len(){
                    if !winner_map[y][x] {
                        unmarked_sum += winner_block[y][x];
                    }
                }
            }
            println!("Result is {} * {} = {}", unmarked_sum, current_number, unmarked_sum * current_number);
            break;
        }
    }
}

pub fn puzzle2(){
    let data = fs::read_to_string("input-day4.txt").expect("error reading input file");
    let mut lines = data.lines();
    let numbers : Vec<i32> = lines.next().unwrap().split(',').map(|n| n.parse::<i32>().unwrap()).collect();
    println!("Drawn numbers are: {:?}", numbers);

    let blocks = get_blocks(lines);
    //println!("blocks: {:?}", blocks);

    let mut block_map : Vec<(&Block, Vec<Vec<bool>>)> = blocks.iter().map(|b| (b, b.iter().map(|y| y.iter().map(|_| false).collect()).collect())).collect();
    let mut winners : Vec<(Block, Vec<Vec<bool>>, i32)> = Vec::new();
    for n in numbers {
        {
            // mark drawn number
            for bm in block_map.iter_mut(){
                let (block, map) = bm;
                mark_number(n, block, map);
                if is_winner((block, map)){
                    if winners.iter().any(|w| w.0 == **block) { // because I'm too lazy at this point. Move winner check outside the iterator in a second loop to directly remove entry instead.
                        println!("skip duplicate entry")
                    } else {
                        let entry = ((*block).clone(), map.clone(), n);
                        winners.push(entry);// need to clone because further iterations might alter the map
                    }
                }
            }
        }
    }

    let (last_block, last_block_map, last_draw) = winners.last().unwrap();
    println!("found last winner drawing {}: {:?}, map: {:?}",last_draw, last_block, last_block_map);
    let mut unmarked_sum = 0;
    for y in 0..last_block_map.len() {
        for x in 0..last_block_map[0].len(){
            if !last_block_map[y][x] {
                unmarked_sum += last_block[y][x];
            }
        }
    }
    println!("Result is {} * {} = {}", unmarked_sum, last_draw, unmarked_sum * last_draw);
}

fn mark_number(n:i32, block:&Block, map:&mut Vec<Vec<bool>>) {
    for y in 0..block.len() {
        for x in 0..block[y].len() {
            if block[y][x] == n {
                (&mut map[y])[x] = true;
                return;
            }
        }
    }
}

fn is_winner((_, map):(&Block, &Vec<Vec<bool>>)) -> bool {
    // check rows
    for row in map {
        if row.iter().all(|b| *b ) {
           return true;
        }
    }

    // check columns
    for x in 0..map[0].len() {
        let column : Vec<bool> = map.iter().flat_map(|row| vec![row[x]]).collect();
        if column.iter().all(|b| *b) {
            return true;
        }
    }
    false
}

fn find_winner<'a>(blocks:&'a Vec<(&Block, Vec<Vec<bool>>)>) -> Option<(&'a Block, &'a Vec<Vec<bool>>)>{
    for (block, map) in blocks {
        // check rows
        for row in map {
            if row.iter().all(|b| *b ) {
                return Some((block, map));
            }
        }

        // check columns
        for x in 0..map[0].len() {
            let column : Vec<bool> = map.iter().flat_map(|row| vec![row[x]]).collect();
            if column.iter().all(|b| *b) {
                return Some((block, map));
            }
        }
    }
    None
}

fn get_blocks(lines:Lines) -> Vec<Block> {
    let mut all_blocks : Vec<Block> = Vec::new();
    let mut current_block : Block = Vec::new();
    for next in lines {
        if next.is_empty() {
            if !current_block.is_empty() {
                all_blocks.push(current_block);
                current_block = Vec::new();
            }
        }
        else {
            let block_line : Vec<i32> = next.split_whitespace().map(|x| x.parse::<i32>().unwrap()).collect();
            current_block.push(block_line);
        }
    }
    all_blocks
}