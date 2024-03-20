#![allow(dead_code, unused_variables)]
use anyhow::Result;
use clap::Parser;
use n_puzzle::heuristics::Heuristic;
use n_puzzle::npuzzle::NPuzzle;
use n_puzzle::solver::Solver;
use std::time::Instant;

#[derive(Parser, Debug)]
struct Args {
    /// Sets the size of the randomly generated n-puzzle
    #[arg(short, long, default_value_t = 3)]
    size: u8,

    /// Finds the optimal solution otherwise greedy
    #[arg(short, long, default_value_t = false)]
    optimal: bool,

    /// Heuristik to choose
    #[arg(value_enum, short = 'H', long, default_value_t = Heuristic::ManhattanDistance)]
    heuristik: Heuristic,

    // File to read an n-puzzle from
    #[arg()]
    file_path: Option<String>,
}

fn main() -> Result<()> {
    let args = Args::parse();

    println!("{:?}", args);

    let npuzzle = if let Some(file_path) = args.file_path {
        NPuzzle::from_file(&file_path)?
    } else {
        NPuzzle::new(args.size)
    };

    let solver = Solver::new(args.optimal, args.heuristik);

    let now = Instant::now();
    let res = solver.solve(npuzzle.clone())?;
    let elapsed = now.elapsed();

    let _solved = Solver::check_solution(npuzzle.clone(), &res)?;
    println!(
        "{}\nPuzzle was solved in {} moves and {:.3?}",
        npuzzle,
        res.len(),
        elapsed,
    );
    Ok(())
}
