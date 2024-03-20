#![allow(dead_code, unused_variables, unused_imports)]
use std::collections::BinaryHeap;
use std::collections::HashMap;
use std::collections::HashSet;

use crate::heuristics;
use crate::heuristics::Heuristic;
use crate::heuristics::HeuristicFn;
use crate::node::Node;
use crate::npuzzle;
use crate::npuzzle::Move;
use crate::npuzzle::NPuzzle;
use anyhow::anyhow;
use anyhow::Result;

pub struct Solver {
    heuristic_fn: HeuristicFn,
    greedy: bool,
}

impl Solver {
    pub fn new(greedy: bool, heuristic: Heuristic) -> Self {
        Solver {
            heuristic_fn: heuristic.get_heuristic_fn(),
            greedy,
        }
    }

    fn reconstruct_path(closed_set: &HashSet<Box<Node>>, goal: Box<Node>) -> Result<Vec<Move>> {
        let mut path = Vec::new();
        let mut last = goal;

        while let Some(node) = closed_set.get(&last) {
            if let Some(movement) = node.movement {
                path.push(movement);

                let inverse = match movement {
                    Move::Up => Move::Down,
                    Move::Down => Move::Up,
                    Move::Left => Move::Right,
                    Move::Right => Move::Left,
                };

                let mut npuzzle = node.npuzzle.clone();
                if npuzzle.apply_move(inverse).is_ok() {
                    last = Box::new(Node::new(npuzzle, 0, 0, None));
                } else {
                    return Err(anyhow!("unable to apply move"));
                }
            } else {
                path.reverse();
                return Ok(path);
            }
        }

        Err(anyhow!("unable to reconstruct path"))
    }

    pub fn solve(&self, npuzzle: NPuzzle) -> Result<Vec<Move>> {
        if !npuzzle.is_solvable() {
            return Err(anyhow!("puzzle is not solveable"));
        }

        let mut open_set = BinaryHeap::new();
        let mut closed_set = HashSet::new();
        let h = (self.heuristic_fn)(&npuzzle);

        open_set.push(Box::new(Node::new(npuzzle, 0, h, None)));

        while let Some(node) = open_set.pop() {
            closed_set.insert(node.clone());
            if node.h == 0 {
                return Solver::reconstruct_path(&closed_set, node);
            }

            let successors = node.get_successors(&self.heuristic_fn);
            for s in successors {
                if !closed_set.contains(&s) {
                    open_set.push(s);
                }
            }
        }

        Err(anyhow!("couldn't reach goal, unsolveable"))
    }

    pub fn check_solution(mut npuzzle: NPuzzle, moves: &Vec<Move>) -> Result<NPuzzle> {
        let mut correct_board: Vec<u8> = (1..npuzzle.size.pow(2)).collect();
        correct_board.push(0);
        for movement in moves {
            npuzzle.apply_move(*movement)?;
        }
        if npuzzle.board == correct_board {
            Ok(npuzzle)
        } else {
            Err(anyhow!("solution does not lead to a correct board"))
        }
    }
}
