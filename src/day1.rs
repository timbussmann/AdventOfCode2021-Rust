use std::{fs, io::BufRead};

pub fn puzzle1() {
    let contents = fs::read_to_string("input.txt").expect("error reading input file");
    let (c, _) : (i32, i32) = contents.lines().map(|line| line.parse().expect("parse error")).fold((0,0), |(count, last), current| {
        if current > last {
            return (count+1, current);
        }
        (count, current)
    });

    // -1 because of the first increase (alternatively, start fold with -1 as in puzzle2)
    println!("First solution is: {}", c - 1);
}

pub fn puzzle2() {
    let numbers : Vec<i32> = fs::read_to_string("input.txt").expect("error reading input file").lines()
        .map(|line| line.parse().expect("parse error")).collect();

    let x1 = numbers.iter();
    let x2 = numbers.iter().skip(1);
    let x3 = numbers.iter().skip(2);

    //let windows : Vec<(&i32, &i32, &i32)> = x1.zip(x2).zip(x3).map(|((a, b), c)| (a, b, c)).collect();

    let (total_increases, _) = x1.zip(x2).zip(x3).map(|((a, b), c)| a + b + c)
    .fold((-1,0), |(count, last), current| { // start fold with -1 because first increment doesn't count
        if current > last {
            return (count + 1, current);
        }
        (count, current)
    });
    println!("Second solution is {}", total_increases);
}