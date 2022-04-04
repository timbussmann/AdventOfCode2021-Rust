use std::{fs, io::BufRead, collections::{HashSet, HashMap}, ops::{Sub, Add, Index, IndexMut}, iter::TakeWhile, fmt::Debug};

pub fn puzzle1(){
    let _ = fs::read_to_string("input-day17.txt").expect("error reading input file");
    //TODO parsing
    
    let min_x = 150;
    let max_x = 171;
    let min_y = -70;
    let max_y: i32 = -129;

    let mut valid = Vec::new();
    for xv in 0..=max_x {
        for xy in max_y..=max_y.abs() {
            let trajectory = Trajectory::new(xv, xy as i32);
            for (cx, cy) in trajectory.into_iter() {
                if cx > max_x || cy < max_y {
                    break; //overshot
                }

                if cx >= min_x && cx <= max_x && cy <= min_y && cy >= max_y {
                    // valid trajectory
                    valid.push((trajectory.x_velocity, trajectory.y_velocity));
                    break;
                }
            }
        }
    }
    
    //get start position with highest y velocity
    valid.sort_by(|(_, y1), (_, y2)| y1.cmp(y2));
    let result = valid.last().unwrap();
    let t = Trajectory::new(result.0, result.1);
    let highest_y = t.into_iter().take_while(|(_, y)| *y > 0).max().unwrap();
    println!("highest y reached by initial trajectory {}, {}: {}", result.0, result.1, highest_y.1);
    assert_eq!(8256, highest_y.1);
    println!("total number of valid trajectories: {}", valid.len());
    assert_eq!(valid.len(), 2326);

}

struct Trajectory {
    x_velocity: i32,
    y_velocity: i32,
}

#[derive(Debug)]
struct Position {
    x_velocity: i32,
    y_velocity: i32,
    current: (i32, i32)
}

impl Trajectory {
    fn new(x_velocity: i32, y_velocity: i32) -> Trajectory {
        Trajectory { x_velocity, y_velocity }
    }
}

impl IntoIterator for &Trajectory {
    type Item = (i32, i32);

    type IntoIter = Position;

    fn into_iter(self) -> Self::IntoIter {
        Position{ x_velocity: self.x_velocity, y_velocity: self.y_velocity, current: (0, 0) }
    }
}

impl Iterator for Position {
    type Item = (i32, i32);

    fn next(&mut self) -> Option<Self::Item> {
        self.current = (self.current.0 + self.x_velocity, self.current.1 + self.y_velocity);
        if self.x_velocity > 0 {
            self.x_velocity -= 1;
        }
        self.y_velocity -= 1;
        Some(self.current)
    }
}

