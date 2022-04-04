use std::{fs, io::BufRead};
use std::str;

pub fn puzzle1() {
    let data = fs::read_to_string("input-day3.txt").expect("error reading input file");
    let bits : Vec<Vec<u32>> = data.lines().map(|l| l.chars().map(|c| c.to_digit(10).unwrap()).collect()).collect();
    let lines = bits.len();
    let counts = bits.iter().fold([0; 12], |mut acc, i| {
        for x in 0..acc.len() {
            if i[x] == 1 {
                acc[x] += 1;
            }
        }
        acc
    });

    let r : [bool; 12] = counts.map(|x| x > (lines / 2));
    println!("{:?}", r);
    let gamma_rate_array = r.map(|x| x as u32);
    let epsilon_rate_array = r.map(|x| !x as u32);
    println!("{:?}", gamma_rate_array);
    println!("{:?}", epsilon_rate_array);
    let gamma_rate_string : String = gamma_rate_array.map(|x| char::from_digit(x, 10).unwrap()).iter().collect();
    let gamma_rate =i32::from_str_radix(gamma_rate_string.as_str(), 2).unwrap();

    let epsilon_rate_string : String = epsilon_rate_array.map(|x| char::from_digit(x, 10).unwrap()).iter().collect();
    let epsilon_rate =i32::from_str_radix(epsilon_rate_string.as_str(), 2).unwrap();

    println!("Result from gamma rate {} and epsilon rate {} is: {}", gamma_rate, epsilon_rate, gamma_rate * epsilon_rate);
}

pub fn puzzle2() {
    let data = fs::read_to_string("input-day3.txt").expect("error reading input file");
    let bits : Vec<Vec<u32>> = data.lines().map(|l| l.chars().map(|c| c.to_digit(10).unwrap()).collect()).collect();

    let mut oxy_values = bits.clone();
    for index in 0..oxy_values[0].len() {

        let flag = oxy_values.iter().fold(0, |acc, v| acc + ((v[index] > 0) as usize));
        let flag = flag >= oxy_values.len() / 2;
        // drainfilter is only in unstable channels
        oxy_values = oxy_values.into_iter().filter(|line| (line[index] != 0) == flag).collect();
        if oxy_values.len() == 1 {
            break;
        }
    }

    let oxy_value_string: String = oxy_values[0].iter().map(|x| char::from_digit(*x, 10).unwrap()).collect();
    let oxy_value = usize::from_str_radix(&oxy_value_string, 2).unwrap();
    println!("oxygen rating value: {}", oxy_value_string);
    println!("oxygen rating value: {}", oxy_value);

    let mut co2_values = bits;
    for index in 0..co2_values[0].len() {

        let flag = co2_values.iter().fold(0, |acc, v| acc + ((v[index] > 0) as usize)) >= (co2_values.len()/2);
        // drainfilter is only in unstable channels
        co2_values = co2_values.into_iter().filter(|line| (line[index] != 0) != flag).collect();
        if co2_values.len() == 1 {
            break;
        }
    }

    let co2_value_string: String = co2_values[0].iter().map(|x| char::from_digit(*x, 10).unwrap()).collect();
    let co2_value = usize::from_str_radix(&co2_value_string, 2).unwrap();
    println!("co2 scrubber value: {}", co2_value_string);
    println!("co2 scrubber rating value: {}", co2_value);

    println!("solution is {}", oxy_value * co2_value);
}