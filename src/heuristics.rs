use crate::npuzzle;
use crate::npuzzle::Move;
use crate::npuzzle::NPuzzle;
use anyhow::anyhow;
use anyhow::Result;
use clap::ValueEnum;
use clap::ValueHint;

pub type HeuristicFn = fn(&NPuzzle) -> i32;

#[derive(Debug, Copy, Clone, ValueEnum)]
pub enum Heuristic {
    /// Manhattan distance
    ManhattanDistance,
    /// Misplaced Tiles
    MisplacedTiles,
}

impl Heuristic {
    pub fn get_heuristic_fn(&self) -> HeuristicFn {
        match self {
            Heuristic::ManhattanDistance => total_manhattan_distance,
            Heuristic::MisplacedTiles => unimplemented!(),
        }
    }
}

// pub fn update_manhattan_distance(npuzzle: &NPuzzle, costs: &mut Vec<u16>) {

// }

// pub fn update_manhattan_distance(npuzzle: &NPuzzle, old: i32, movement: Move) -> i32 {
//     let goal_index = value as usize - 1;
//     let value_index_old = npuzzle.find_index(value).unwrap();
//     let old_man = manhattan_distance(npuzzle.size as usize, value_index_old, goal_index);
//     let value_index_new = match movement {
//         Move::Up => value_index_old - npuzzle.size as usize,
//         Move::Down => value_index_old + npuzzle.size as usize,
//         Move::Left => value_index_old - 1,
//         Move::Right => value_index_old + 1,
//     };
//     let new_man = manhattan_distance(npuzzle.size as usize, value_index_new, goal_index);
//     old - old_man + new_man
// }

pub fn total_manhattan_distance(npuzzle: &NPuzzle) -> i32 {
    // static mut COUNTER: i32 = 0;
    // use std::time::Instant;

    // let now = Instant::now();
    let mut total_distance = 0;
    for (index, value) in (1..(npuzzle.size.pow(2))).enumerate() {
        let value_index = npuzzle.find_index(value).unwrap();
        total_distance += manhattan_distance(npuzzle.size as usize, value_index, index);
    }

    // let elapsed = now.elapsed();
    // unsafe {
    //     COUNTER += 1;
    //     println!("{:?} {}", elapsed, COUNTER);
    // }
    total_distance
}

pub fn manhattan_distance(size: usize, value_index: usize, goal_index: usize) -> i32 {
    let goal_x = goal_index % size;
    let goal_y = goal_index / size;
    let value_x = value_index % size;
    let value_y = value_index / size;
    (goal_x as i32 - value_x as i32).abs() + (goal_y as i32 - value_y as i32).abs()
}

// pub fn linear_conflict(npuzzle: &NPuzzle) -> i32 {}

// pub fn row_conflicts(npuzzle: &NPuzzle, row: u8) -> i32 {
//     let size = npuzzle.size;
//     let board = &npuzzle.board;
//     for i in (size * row)..(size * row + size) {
//         let val = board[i as usize];
//         for j in (i + 1)..(size * row + size) {

//         }
//     }
//     unimplemented!()
// }
