use anyhow::anyhow;
use anyhow::Result;
use rand::seq::SliceRandom;
use std::fmt;
use std::fmt::Display;

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum Move {
    Up,
    Down,
    Left,
    Right,
}

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq)]
pub struct Point {
    pub x: u8,
    pub y: u8,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct NPuzzle {
    pub size: u8,
    pub board: Vec<u8>,
    pub blank_pos: Point,
}

impl NPuzzle {
    pub fn new(size: u8) -> Self {
        NPuzzle::generate_random(size, true)
    }

    pub fn from_file(file_path: &str) -> Result<Self> {
        let content = std::fs::read_to_string(file_path)?;
        let mut lines = content.lines();

        while let Some(line) = lines.next() {
            let (data, _) = split_line_with_comment(line);
            if let Some(data) = data {
                let size = data.split_whitespace().nth(0).unwrap().parse::<u8>()?;
                let mut board: Vec<u8> = Vec::with_capacity((size as usize).pow(2));
                while let Some(puzzle_line) = lines.next() {
                    if let (Some(puzzle_line), _) = split_line_with_comment(puzzle_line) {
                        let values: Vec<u8> = puzzle_line
                            .split_whitespace()
                            .map(|s| s.parse::<u8>().unwrap())
                            .collect();
                        if values.len() != size as usize {
                            return Err(anyhow!("row sizes differ"));
                        }
                        for val in values {
                            board.push(val);
                        }
                    } else {
                        return Err(anyhow!("expected puzzle input"));
                    }
                }
                if let Some(blank_pos) = NPuzzle::find_blank(&board, size) {
                    return Ok(NPuzzle {
                        size,
                        board,
                        blank_pos,
                    });
                } else {
                    return Err(anyhow!("No empty tile in puzzle"));
                }
            }
        }
        Err(anyhow!("parsing error"))
    }

    fn find_blank(board: &Vec<u8>, size: u8) -> Option<Point> {
        let mut blank_pos = Point::default();
        for (i, &tile) in board.iter().enumerate() {
            if tile == 0 {
                blank_pos.x = i as u8 % size;
                blank_pos.y = i as u8 / size;
                return Some(blank_pos);
            }
        }
        None
    }

    pub fn generate_random(size: u8, solvable: bool) -> Self {
        let total_elements = (size as usize).pow(2);
        let mut board: Vec<u8> = (0..total_elements as u8).collect();
        board.shuffle(&mut rand::thread_rng());

        let blank_pos = NPuzzle::find_blank(&board, size).unwrap();
        let mut npuzzle = NPuzzle {
            size,
            board,
            blank_pos,
        };

        let is_solveable = npuzzle.is_solvable();

        if solvable != is_solveable {
            if npuzzle.board[0] != 0 && npuzzle.board[1] != 0 {
                npuzzle.board.swap(0, 1);
            } else {
                npuzzle.board.swap(total_elements - 1, total_elements - 2);
            }
        }

        npuzzle
    }

    pub fn find_index(&self, value: u8) -> Option<usize> {
        self.board.iter().position(|&x| x == value)
    }

    fn inversion_count(&self) -> u32 {
        let mut inversion_count = 0;

        for i in 0..self.board.len() {
            for j in (i + 1)..self.board.len() {
                let val_i = self.board[i];
                let val_j = self.board[j];
                if val_i != 0 && val_j != 0 && val_i > val_j {
                    inversion_count = inversion_count + 1;
                }
            }
        }

        inversion_count
    }

    pub fn is_solvable(&self) -> bool {
        let inversion_count = self.inversion_count();
        let size = self.size;

        if size % 2 == 1 {
            return inversion_count % 2 == 0;
        } else {
            if self.blank_pos.y % 2 == 0 {
                return inversion_count % 2 == 1;
            } else {
                return inversion_count % 2 == 0;
            }
        }
    }

    pub fn can_move(&self, movement: Move) -> bool {
        match movement {
            Move::Up => self.blank_pos.y != self.size - 1,
            Move::Down => self.blank_pos.y != 0,
            Move::Left => self.blank_pos.x != self.size - 1,
            Move::Right => self.blank_pos.x != 0,
        }
    }

    pub fn apply_move(&mut self, movement: Move) -> Result<()> {
        if !self.can_move(movement) {
            return Err(anyhow!("unable to apply move"));
        }

        let blank_pos = self.blank_pos;

        let (swap_x, swap_y) = match movement {
            Move::Up => (blank_pos.x, blank_pos.y + 1),
            Move::Down => (blank_pos.x, blank_pos.y - 1),
            Move::Left => (blank_pos.x + 1, blank_pos.y),
            Move::Right => (blank_pos.x - 1, blank_pos.y),
        };

        let swap_index = swap_y as usize * self.size as usize + swap_x as usize;
        let blank_index = blank_pos.y as usize * self.size as usize + blank_pos.x as usize;

        self.board.swap(blank_index, swap_index);
        self.blank_pos.x = swap_x;
        self.blank_pos.y = swap_y;

        Ok(())
    }
}

impl Display for NPuzzle {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}\n\n", "-".repeat(self.size as usize * 4 + 3))?;
        for i in 0..self.size as usize {
            for j in 0..self.size as usize {
                let value = self.board[i * self.size as usize + j];
                write!(
                    f,
                    "{:>4}",
                    if value == 0 {
                        "■".to_string()
                    } else {
                        value.to_string()
                    }
                )?;
            }
            write!(f, "\n\n")?;
        }
        Ok(())
    }
}

fn split_line_with_comment(line: &str) -> (Option<&str>, Option<&str>) {
    let (data, comment) = line.split_once('#').unwrap_or((line, ""));
    (
        if data.is_empty() {
            None
        } else {
            Some(data.trim())
        },
        if comment.is_empty() {
            None
        } else {
            Some(comment.trim())
        },
    )
}
