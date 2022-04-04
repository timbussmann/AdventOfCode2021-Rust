use std::{fs, io::BufRead, collections::{HashSet, HashMap}, ops::{Sub, Index, IndexMut, RangeBounds}, iter::TakeWhile, fmt::Debug};

type State = Vec<isize>;

#[derive(Debug)]
#[derive(PartialEq)]
enum Instruction {
    Input(char),
    Add(char, Variable),
    Multiply(char, Variable),
    Divide(char, Variable),
    Modulo(char, Variable),
    Equal(char, Variable)
}

#[derive(Debug)]
#[derive(PartialEq)]
enum Variable {
    Char(char),
    Int(isize)
}

impl Instruction {
    fn execute(&self, state: &mut State) {
        match self {
            Instruction::Add(char, var) => {
                let value2 = variable_value(var, state);
                state[state_index(char)] += value2;
            },
            Instruction::Multiply(char, var) => {
                let value2 = variable_value(var, state);
                state[state_index(char)] *= value2;
            },
            Instruction::Divide(char, var) => {
                let value2 = variable_value(var, state);
                state[state_index(char)] /= value2;
            },
            Instruction::Modulo(char, var) => {
                let value2 = variable_value(var, state);
                state[state_index(char)] %= value2;
            },
            Instruction::Equal(char, var) => {
                let value2 = variable_value(var, state);
                let index = state_index(char);
                state[index] = if state[index] == value2 { 1 } else { 0 };
            },
            _ => panic!()
        }
    }
}

// runs in 17.6 seconds on windows
// runs in 13 seconds on docker dev container
pub fn puzzle1() {
    let input = fs::read_to_string("input-day24.txt").expect("error reading input file");
    let instructions: Vec<Instruction> = input.lines().map(|l| parse_line(l)).collect();

    let max_z: isize = 26i64.pow(5) as isize;
    let mut states: Vec<(State, isize)> = Vec::from([(vec![0,0,0,0], 0)]);
    for op in instructions {
        match op {
            Instruction::Input(char) => {
                let mut new_states = HashMap::new();
                for (state, number) in states {
                    if state[3] > max_z {
                        // to big to be able ever get back to 0
                        continue;
                    }
                    for i in (1..=9).rev() {
                        let mut new_state = vec![0; 4];
                        new_state[3] = state[3]; // copy z
                        new_state[state_index(&char)] = i;
                        let new_number = number * 10 + i;
                        new_states.entry(new_state)
                            .and_modify(|n| *n = if new_number > *n { new_number } else { *n })
                            .or_insert(new_number);
                    }
                }
                states = new_states.into_iter().map(|kvp| (kvp.0, kvp.1)).collect();
                println!("finished insert with {} states.", states.len());
            },
            _ => {
                for (state, _) in &mut states {
                    op.execute(state);
                }
                print!(".");
            }
        }
    }

    let result = states.into_iter().filter(|x| x.0[3] == 0).map(|x| x.1).max().unwrap();
    dbg!(result);
}

pub fn puzzle2() {
    let input = fs::read_to_string("input-day24.txt").expect("error reading input file");
    let instructions: Vec<Instruction> = input.lines().map(|l| parse_line(l)).collect();

    let max_z: isize = 26i64.pow(5) as isize;
    let mut states: Vec<(State, isize)> = Vec::from([(vec![0,0,0,0], 0)]);
    for op in instructions {
        match op {
            Instruction::Input(char) => {
                let mut new_states = HashMap::new();
                for (state, number) in states {
                    if state[3] > max_z {
                        // to big to be able ever get back to 0
                        continue;
                    }
                    for i in (1..=9).rev() {
                        let mut new_state = vec![0; 4];
                        new_state[3] = state[3]; // copy z
                        new_state[state_index(&char)] = i;
                        let new_number = number * 10 + i;
                        new_states.entry(new_state)
                            .and_modify(|n| *n = isize::min(*n, new_number))
                            .or_insert(new_number);
                    }
                }
                states = new_states.into_iter().map(|kvp| (kvp.0, kvp.1)).collect();
                println!("finished insert with {} states.", states.len());
            },
            _ => {
                for (state, _) in &mut states {
                    op.execute(state);
                }
            }
        }
    }

    let result = states.into_iter().filter(|x| x.0[3] == 0).map(|x| x.1).max().unwrap();
    dbg!(result);
}

fn state_index(char: &char) -> usize {
    match char {
        'w' => 0,
        'x' => 1,
        'y' => 2,
        'z' => 3,
        _ => panic!()
    }
}

fn variable_value(var: &Variable, state: &State) -> isize {
    match var {
        Variable::Char(char) => state[state_index(char)],
        Variable::Int(i) => *i
    }
}

fn parse_line(line: &str) -> Instruction {
    let parts: Vec<&str> = line.split_ascii_whitespace().collect();
    match parts[0] {
        "inp" => Instruction::Input(get_char(parts[1])),
        "add" => Instruction::Add(get_char(parts[1]), parse_variable(parts[2])),
        "mul" => Instruction::Multiply(get_char(parts[1]), parse_variable(parts[2])),
        "div" => Instruction::Divide(get_char(parts[1]), parse_variable(parts[2])),
        "mod" => Instruction::Modulo(get_char(parts[1]), parse_variable(parts[2])),
        "eql" => Instruction::Equal(get_char(parts[1]), parse_variable(parts[2])),
        _ => panic!("unknown instruction!")
    }
}

fn get_char(variable: &str) -> char{
    variable.chars().next().unwrap()
}

fn parse_variable(variable: &str) -> Variable {
    if let Ok(number) = variable.parse::<isize>() {
        return Variable::Int(number);
    }
    Variable::Char(get_char(variable))
}

#[cfg(test)]
mod tests{
    use crate::day24::{Instruction, Variable};

    use super::parse_line;

    #[test]
    fn parse_input(){
        let result = parse_line("inp w");
        assert_eq!(result, Instruction::Input('w'));
    }

    #[test]
    fn parse_add(){
        let result = parse_line("add z y");
        assert_eq!(result, Instruction::Add('z', Variable::Char('y')));

        let result = parse_line("add z 42");
        assert_eq!(result, Instruction::Add('z', Variable::Int(42)));
    }

    #[test]
    fn execute_add(){
        let add_v = Instruction::Add('x', Variable::Char('y'));
        let add_i = Instruction::Add('x', Variable::Int(10));

        let mut state = vec![0,0, 20, 0];
        add_v.execute(&mut state);
        add_i.execute(&mut state);

        assert_eq!(30, state[1]);
    }

    #[test]
    fn parse_multiply(){
        let result = parse_line("mul z y");
        assert_eq!(result, Instruction::Multiply('z', Variable::Char('y')));

        let result = parse_line("mul z 42");
        assert_eq!(result, Instruction::Multiply('z', Variable::Int(42)));
    }

    #[test]
    fn execute_multiply(){
        let add_v = Instruction::Multiply('x', Variable::Char('y'));
        let add_i = Instruction::Multiply('x', Variable::Int(10));

        let mut state = vec![0, 1, 100, 0];
        add_v.execute(&mut state);
        add_i.execute(&mut state);

        assert_eq!(100 * 10, state[1]);
    }

    #[test]
    fn parse_div(){
        let result = parse_line("div z y");
        assert_eq!(result, Instruction::Divide('z', Variable::Char('y')));

        let result = parse_line("div z 42");
        assert_eq!(result, Instruction::Divide('z', Variable::Int(42)));
    }

    #[test]
    fn execute_divide(){
        let add_v = Instruction::Divide('x', Variable::Char('y'));
        let add_i = Instruction::Divide('x', Variable::Int(10));

        let mut state = vec![0, 100, 10, 0];
        add_v.execute(&mut state);
        add_i.execute(&mut state);

        assert_eq!(1, state[1]);
    }

    #[test]
    fn parse_mod(){
        let result = parse_line("mod z y");
        assert_eq!(result, Instruction::Modulo('z', Variable::Char('y')));

        let result = parse_line("mod z 42");
        assert_eq!(result, Instruction::Modulo('z', Variable::Int(42)));
    }

    #[test]
    fn execute_mod(){
        let add_v = Instruction::Modulo('x', Variable::Char('y'));
        let add_i = Instruction::Modulo('x', Variable::Int(5));

        let mut state = vec![0, 109, 10, 0];
        add_v.execute(&mut state);
        add_i.execute(&mut state);

        assert_eq!(4, state[1]);
    }

    #[test]
    fn parse_eq(){
        let result = parse_line("eql z y");
        assert_eq!(result, Instruction::Equal('z', Variable::Char('y')));

        let result = parse_line("eql z 42");
        assert_eq!(result, Instruction::Equal('z', Variable::Int(42)));
    }

    #[test]
    fn execute_eq(){
        let add_v = Instruction::Equal('w', Variable::Char('x'));
        let add_i = Instruction::Equal('y', Variable::Int(5));

        let mut state = vec![42, 42, 3, 0];
        add_v.execute(&mut state);
        add_i.execute(&mut state);

        assert_eq!(vec![1, 42, 0 , 0], state);
    }
}
