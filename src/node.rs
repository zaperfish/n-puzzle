use super::npuzzle::NPuzzle;
use crate::heuristics::HeuristicFn;
use crate::npuzzle::Move;
use std::cmp::Ordering;
use std::hash::{Hash, Hasher};

#[derive(Debug, Clone)]
pub struct Node {
    pub npuzzle: NPuzzle,
    // pub f: i32,
    pub g: i32,
    pub h: i32,
    pub movement: Option<Move>,
}

impl Node {
    pub fn new(npuzzle: NPuzzle, g: i32, h: i32, movement: Option<Move>) -> Self {
        Node {
            npuzzle,
            g,
            h,
            movement,
        }
    }

    fn get_successor(&self, movement: Move, heuristic_fn: &HeuristicFn) -> Option<Box<Node>> {
        if !self.npuzzle.can_move(movement) {
            return None;
        }

        let mut successor_npuzzle = self.npuzzle.clone();

        if successor_npuzzle.apply_move(movement).is_ok() {
            let new_h = heuristic_fn(&successor_npuzzle);
            Some(Box::new(Node::new(
                successor_npuzzle,
                self.g + 0,
                new_h,
                Some(movement),
            )))
        } else {
            None
        }
    }

    pub fn get_successors(&self, heuristic_fn: &HeuristicFn) -> Vec<Box<Node>> {
        let movements = [Move::Up, Move::Down, Move::Left, Move::Right];

        movements
            .iter()
            .filter_map(|&movement| self.get_successor(movement, heuristic_fn))
            .collect()
    }
}

impl PartialOrd for Node {
    fn partial_cmp(&self, other: &Node) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for Node {
    fn cmp(&self, other: &Node) -> Ordering {
        (other.g + other.h).cmp(&(self.g + self.h))
    }
}

impl PartialEq for Node {
    fn eq(&self, other: &Node) -> bool {
        self.npuzzle.board == other.npuzzle.board
    }
}

impl Eq for Node {}

impl Hash for Node {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.npuzzle.board.hash(state);
    }
}
