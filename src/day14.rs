use std::{fs, io::BufRead, collections::{HashSet, HashMap}, ops::{Sub, Add}};

pub fn puzzle1(){
    let input = fs::read_to_string("input-day14.txt").expect("error reading input file");
    let lines: Vec<&str> = input.lines().collect();

    let mut template: String = lines[0].to_string();
    //dbg!(template);

        let mut rule_map: HashMap<String, String> = HashMap::new();
    for rule in lines[2..].iter()
    {
        let split: Vec<&str> = rule.split(" -> ").collect();
        rule_map.insert(split[0].to_string(), split[1].to_string());
    }
    //dbg!(rule_map);

    for _ in 0..10 {
        template = process(template, &rule_map);
    }

    //dbg!(template);

    // count chars
    let mut char_map: HashMap<char, usize> = HashMap::new();
    for c in template.chars() {
        if let Some(v) = char_map.get_mut(&c) {
            *v += 1;
        } else{
            char_map.insert(c, 1);
        }
    }

    // get max and min:
    let (_, max) = char_map.iter().max_by(|(_, a), (_, b)| a.cmp(b)).unwrap();
    let (_, min) = char_map.iter().min_by(|(_, a), (_, b)| a.cmp(b)).unwrap();
    println!("{}", max - min);
}

fn process(input: String, rule_map: &HashMap<String, String>) -> String {
    let pairs: Vec<(char, char)> = input.chars().zip(input.chars().skip(1)).collect();
    let mut output = String::new();
    for (a, b) in pairs {
        let key: String = format!("{}{}", a, b);
        let c = rule_map.get(&key).unwrap();
        output.push(a);
        output.push_str(c);
    }
    // add last char
    output.push(input.chars().last().unwrap());
    output
}

pub fn puzzle2(){
    let input = fs::read_to_string("input-day14.txt").expect("error reading input file");
    let lines: Vec<&str> = input.lines().collect();

    let template: Vec<char> = lines[0].chars().collect();

    let mut rule_map: HashMap<(char, char), char> = HashMap::new();
    for rule in lines[2..].iter()
    {
        let split: Vec<&str> = rule.split(" -> ").collect();
        rule_map.insert({
            let mut chars = split[0].chars();
            (chars.next().unwrap(), chars.next().unwrap())
        }, split[1].chars().next().unwrap());
    }

    let mut counts: HashMap<char, u64> = HashMap::new();

    counts.insert(template[0], 1);
    for i in 0..template.len() {
        *counts.entry(template[i]).or_insert(0) += 1;
    }

    let mut pairs: HashMap<(char, char), u64> = HashMap::new();
    for i in 1..template.len() {
        *pairs.entry((template[i-1], template[i])).or_insert(0) += 1;
    }

    for _ in 0..40 {
        let mut new_pairs : HashMap<(char, char), u64> = HashMap::new();
        for (pair, count) in pairs.iter() {
            let new_char = rule_map.get(pair).unwrap();
            *counts.entry(*new_char).or_insert(0) += count;
            *new_pairs.entry((pair.0, *new_char)).or_insert(0) += count;
            *new_pairs.entry((*new_char, pair.1)).or_insert(0) += count;
        }
        pairs = new_pairs;
    }

    // get max and min:
    let (_, max) = counts.iter().max_by(|(_, a), (_, b)| a.cmp(b)).unwrap();
    let (_, min) = counts.iter().min_by(|(_, a), (_, b)| a.cmp(b)).unwrap();
    println!("{}", max - min);
}

fn expand(a: char, b: char, rule_map: &HashMap<String, char>, counts: &mut HashMap<char, u64>, depth: usize) {
  
    let key: String = format!("{}{}", a, b);
    let middle = rule_map.get(&key).unwrap();

    if let Some(count) = counts.get_mut(middle)  {
        *count += 1;
    } else {
        counts.insert(b, 1);
    }

    if depth == 39 {
        return;
    }

    let d2 = depth + 1;
    
    expand(a, *middle, rule_map, counts, d2);
    expand(*middle, b, rule_map, counts, d2);
}