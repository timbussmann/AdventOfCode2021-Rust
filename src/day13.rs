use std::{fs, io::BufRead, collections::{HashSet, HashMap}};

pub fn puzzle1(){
    let input = fs::read_to_string("input-day13.txt").expect("error reading input file");
    let dot_strings: Vec<&str> = input.lines().take_while(|l| !l.is_empty()).collect();
    let dots: Vec<(usize, usize)>= dot_strings.iter().map(|l| {
        let parts: Vec<&str> = l.split(',').collect();
        (parts[0].parse().unwrap(), parts[1].parse().unwrap())
    }).collect();
    
    let (max_x, _) = dots.iter().max_by(|(a, _), (b, _)| a.cmp(b)).unwrap();
    let (_, max_y) = dots.iter().max_by(|(_, a), (_, b)| a.cmp(b)).unwrap();
    dbg!(max_x);
    dbg!(max_y);

    let mut sheet = vec![vec![false; *max_x + 1]; *max_y + 1];

    for (px, py) in dots {
        sheet[py][px] = true;
    }

    // first fold:
    // fold along x=655
    const FOLD: usize = 655;

    for x_target in 0..FOLD  {
        let x_source = FOLD + FOLD - x_target;
        for y in 0..sheet.len() {
            if !sheet[y][x_target] {
                sheet[y][x_target] = sheet[y][x_source];
            }
        }
    }

    // shorten vector
    for y in sheet.iter_mut() {
        y.resize(FOLD, false);
    }

    let number_of_dots: usize = sheet.iter().map(|l| l.iter().map(|x| *x as usize).sum::<usize>()).sum();
    println!("Result is {}", number_of_dots);
}

pub fn puzzle2(){
    let input = fs::read_to_string("input-day13.txt").expect("error reading input file");
    let dot_strings: Vec<&str> = input.lines().take_while(|l| !l.is_empty()).collect();
    let dots: Vec<(usize, usize)>= dot_strings.iter().map(|l| {
        let parts: Vec<&str> = l.split(',').collect();
        (parts[0].parse().unwrap(), parts[1].parse().unwrap())
    }).collect();
    
    let (max_x, _) = dots.iter().max_by(|(a, _), (b, _)| a.cmp(b)).unwrap();
    let (_, max_y) = dots.iter().max_by(|(_, a), (_, b)| a.cmp(b)).unwrap();
    dbg!(max_x);
    dbg!(max_y);

    let mut sheet = vec![vec![false; *max_x + 1]; *max_y + 1];

    for (px, py) in dots {
        sheet[py][px] = true;
    }

    fold_x(655, &mut sheet);
    fold_y(447, &mut sheet);
    fold_x(327, &mut sheet);
    fold_y(223, &mut sheet);
    fold_x(163, &mut sheet);
    fold_y(111, &mut sheet);
    fold_x(81, &mut sheet);
    fold_y(55, &mut sheet);
    fold_x(40, &mut sheet);
    fold_y(27, &mut sheet);
    fold_y(13, &mut sheet);
    fold_y(6, &mut sheet);

    for line in sheet {
        let chars = line.iter().map(|f| if *f { '#' } else { ' ' }).collect::<String>();
        println!("{}", chars);
    }
}

fn fold_x(pos: usize, sheet: &mut Vec<Vec<bool>>) {
    for x_target in 0..pos  {
        let x_source = pos + pos - x_target;
        for y in 0..sheet.len() {
            if !sheet[y][x_target] {
                sheet[y][x_target] = sheet[y][x_source];
            }
        }
    }

    // shorten vector
    for y in sheet.iter_mut() {
        y.resize(pos, false);
    }
}

fn fold_y(pos: usize, sheet: &mut Vec<Vec<bool>>) {
    let l = (sheet.len() - pos) - 1;
    println!("len {} with pos {} gives range {}", sheet.len(), pos, l);
    for offset in 1..=l  {
                let y_target = pos - offset;
        let y_source = pos + offset;
        for x in 0..sheet[y_target].len() {
            if !sheet[y_target][x] {
                sheet[y_target][x] = sheet[y_source][x];
            }
        }
    }

    // shorten vector
    sheet.resize(pos, vec![]);
}
