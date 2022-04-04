use std::{fs, io::BufRead, collections::{HashSet, HashMap}, ops::{Sub, Add, Index, IndexMut, RangeBounds}, iter::TakeWhile, fmt::Debug};

#[derive(Debug)]
struct DeterministicDice {
    current: u32,
    rolls: u32
}

impl DeterministicDice {
    fn new() -> Self {
        DeterministicDice { current: 0, rolls: 0 }
    }
}

impl Iterator for DeterministicDice {
    type Item = u32;

    fn next(&mut self) -> Option<Self::Item> {
        self.rolls += 1;
        self.current += 1;

        if self.current == 101 {
            self.current = 1;
        }

        Some(self.current)
    }
}

pub fn puzzle1() {
    let input = fs::read_to_string("input-day21.txt").expect("error reading input file");
    let lines: Vec<&str> = input.lines().collect();
    let p1_start = lines[0].chars().last().unwrap().to_digit(10).unwrap();
    let p2_start = lines[1].chars().last().unwrap().to_digit(10).unwrap();

    let mut dice = DeterministicDice::new();

    let mut p1 = (p1_start - 1, 0); //-1 position becase the board internally is from 0 to 9 instead 1 to 10
    let mut p2 = (p2_start - 1, 0);
    let mut current_player = &mut p1;
    let mut next_player = &mut p2;

    loop {
            // roll three times:
            let steps: u32 = (&mut dice).take(3).sum();
            let new_pos = (current_player.0 + steps) % 10;
            (*current_player).0 = new_pos;
            current_player.1 += new_pos + 1; // add 1 because the board scores go from 1 to 10

            //dbg!(&current_player);

            // check for win
            if current_player.1 >= 1000 {
                break; // current player won
            }

            // change player
            std::mem::swap(&mut next_player, &mut current_player);
    }

    dbg!(current_player);
    let result = next_player.1 * dice.rolls;
    println!("Result is {}", result);
}

pub fn puzzle2() {
    let input = fs::read_to_string("input-day21.txt").expect("error reading input file");
    let lines: Vec<&str> = input.lines().collect();
    let p1_start: usize = lines[0].chars().last().unwrap().to_digit(10).unwrap() as usize;
    let p2_start: usize = lines[1].chars().last().unwrap().to_digit(10).unwrap() as usize;

    let mut win_counter = vec![0, 0];
    let player1 = Player::new(0, p1_start - 1);
    let player2 = Player::new(1, p2_start - 1);

    dbg!(get_rolls());
    play(player1, player2, &mut win_counter);

    dbg!(win_counter);
}

#[derive(Debug)]
struct Player {
    score: usize,
    score_counter: usize,
    player_number: usize,
    position: usize
}

impl Player {
    fn new(id: usize, start_pos: usize) -> Self{
        Player { score: 0, score_counter: 1, position: start_pos, player_number: id }
    }
}

fn play(player: Player, next_player: Player, win_counter: &mut Vec<u64>) {
    for roll in get_rolls() {
        let new_pos = (player.position + roll.0) % 10;
        let new_score = player.score + new_pos + 1;

        let player = Player { position: new_pos, score: new_score, score_counter: player.score_counter * roll.1, ..player};

        if new_score >= 21 {
            win_counter[player.player_number] += (player.score_counter * next_player.score_counter) as u64;
        } else{
            play(Player { ..next_player }, player, win_counter);
        }
    }
}

fn get_rolls() -> HashMap<usize, usize> {
    let rolls = [1, 2, 3];
    let permutations: Vec<(usize, usize, usize)> = rolls.iter().flat_map(|i1|{
        rolls.iter().flat_map(|i2| {
            rolls.iter().map(|i3| (*i1, *i2, *i3)).collect::<Vec<(usize, usize, usize)>>()
        }).collect::<Vec<(usize, usize, usize)>>()
    }).collect();

    let mut aggregated: HashMap<usize, usize> = HashMap::new();
    for (a, b, c) in permutations {
        let sum = a + b + c;
        *aggregated.entry(sum).or_insert(0) += 1;
    }
    aggregated
}