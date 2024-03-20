use crate::npuzzle::NPuzzle;
use clap::ValueEnum;

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

pub fn total_manhattan_distance(npuzzle: &NPuzzle) -> i32 {
    let mut total_distance = 0;
    for (index, value) in (1..(npuzzle.size.pow(2))).enumerate() {
        let value_index = npuzzle.find_index(value).unwrap();
        total_distance += manhattan_distance(npuzzle.size as usize, value_index, index);
    }

    total_distance
}

pub fn manhattan_distance(size: usize, value_index: usize, goal_index: usize) -> i32 {
    let goal_x = goal_index % size;
    let goal_y = goal_index / size;
    let value_x = value_index % size;
    let value_y = value_index / size;
    (goal_x as i32 - value_x as i32).abs() + (goal_y as i32 - value_y as i32).abs()
}
