use std::{fs, io::BufRead, collections::{HashSet, HashMap}, ops::{Sub, Add, Index, IndexMut}};

pub fn puzzle1(){
    let input = fs::read_to_string("input-day15.txt").expect("error reading input file");
    let mut map: Vec<Vec<(usize, usize, bool)>> = // node path cost / total sitance path / visited / connecting-position
    input.lines()
            .map(|l| l
            .chars()
                .map(|c| (c.to_digit(10).unwrap() as usize, usize::MAX, false))
                .collect())
            .collect();

    map[0][0].1 = 0;

    let max_x = map[0].len() - 1;
    let max_y = map.len() - 1;
    
    let mut current_pos = (0,0);
    loop {
        let (_, shortest_path, _) = map[current_pos.1][current_pos.0];
        
        let next = get_adjacent_positions(current_pos.0, current_pos.1, max_x, max_y);
        for (x, y) in next {
            let mut n_node = &mut map[y][x];
            if n_node.2 {
                continue; // ignore already visited nodes
            }
            let distance = shortest_path + n_node.0;
            if distance < n_node.1 {
                //update node
                n_node.1 = distance;
            }
        }
    
        // mark node as visited
        let z = &mut map[current_pos.1][current_pos.0];
        z.2 = true;
        
        if current_pos == (max_y, max_x) {
            // completed
            break;
        }
    
        // select next node
        let mut next_node_distance = usize::MAX;
        for y in 0..=max_y {
            for x in 0..=max_x {
                let n = map[y][x];
                if n.2 {
                    continue;
                }
    
                if n.1 < next_node_distance {
                    current_pos = (x, y);
                    next_node_distance = n.1;
                }
            }
        }
    }

    dbg!(map[max_y][max_x]);
}

pub fn puzzle2() {
    let input = fs::read_to_string("input-day15.txt").expect("error reading input file");
    let mut map: Vec<Vec<(usize, usize, bool)>> = // node path cost / total sitance path / visited / connecting-position
    input.lines()
            .map(|l| l
            .chars()
                .map(|c| (c.to_digit(10).unwrap() as usize, usize::MAX, false))
                .collect())
            .collect();



    map = map.iter().map(|l| {
        let mut new = l.clone();
        for i in 1..5 {
            for elem in l {
                let mut new_elem = *elem;
                let overflow = elem.0 + i;
                new_elem.0 = if overflow > 9 { overflow - 9} else { overflow };
                new.push(new_elem);
            }
        }
        new
    }).collect();

    
    let mut map2 = map.clone();
    for i in 1..5 {
        for line in map.iter() {
            let new_line = line.iter().map(|elem|{
                let mut new_elem = *elem;
                let overflow = elem.0 + i;
                new_elem.0 = if overflow > 9 { overflow - 9} else { overflow };
                new_elem
            }).collect();
            map2.push(new_line);
        }
    }
   
    map = map2;
    

    map[0][0].1 = 0;

    let max_x = map[0].len() - 1;
    let max_y = map.len() - 1;
    dbg!(max_x);
    dbg!(max_y);
    
    let mut current_pos = (0,0);
    loop {
        let (_, shortest_path, _) = map[current_pos.1][current_pos.0];
        
        let next = get_adjacent_positions(current_pos.0, current_pos.1, max_x, max_y);
        for (x, y) in next {
            let mut n_node = &mut map[y][x];
            if n_node.2 {
                continue; // ignore already visited nodes
            }
            let distance = shortest_path + n_node.0;
            if distance < n_node.1 {
                //update node
                n_node.1 = distance;
            }
        }
    
        // mark node as visited
        let z = &mut map[current_pos.1][current_pos.0];
        z.2 = true;
        
        if current_pos == (max_y, max_x) {
            // completed
            break;
        }
    
        // select next node
        let mut next_node_distance = usize::MAX;
        for y in 0..=max_y {
            for x in 0..=max_x {
                let n = map[y][x];
                if n.2 {
                    continue;
                }
    
                if n.1 < next_node_distance {
                    current_pos = (x, y);
                    next_node_distance = n.1;
                }
            }
        }
    }

    dbg!(map[max_y][max_x]);
}

fn get_adjacent_positions(x: usize, y: usize, max_x: usize, max_y: usize) -> Vec<(usize, usize)> {
    let mut result: Vec<(usize, usize)> = Vec::new();
    if x < max_x {
        result.push((x + 1, y));
    }
    if x > 0 {
        result.push((x - 1, y));
    }
    if y < max_y {
        result.push((x, y + 1));
    }
    if y > 0 {
        result.push((x, y - 1));
    }
    result
}