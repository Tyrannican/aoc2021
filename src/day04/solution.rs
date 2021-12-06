use super::{read_file, Solve};
use std::collections::HashMap;

#[derive(Debug, Copy, Clone)]
struct Position {
    value: i32,
    marked: bool
}

#[derive(Debug, Copy, Clone)]
struct Coord {
    x: usize,
    y: usize
}

#[derive(Debug, Clone)]
struct Board {
    positions: Vec<Vec<Position>>,
    idx: HashMap<i32, Coord>
}

impl Board {
    pub fn new(input: &str) -> Self {
        let mut board: Vec<Vec<Position>> = Vec::new();
        let mut idx: HashMap<i32, Coord> = HashMap::new();

        for (x, item) in input.split("\n").enumerate() {
            let mut row: Vec<Position> = Vec::new();
            
            let temp = item.replace("  ", " ");
            let trimmed= temp.trim();

            for (y, number) in trimmed.split(" ").enumerate() {
                let value = number.parse::<i32>().unwrap();
                row.push(Position{ value, marked: false });
                idx.insert(value, Coord{ x, y });
            }

            board.push(row);
        }

        Self {
            positions: board,
            idx
        }
    }

    pub fn mark(&mut self, idx: &i32) {
        match self.idx.get(idx) {
            Some(coord) => {
                self.positions[coord.x][coord.y].marked = true;
            },
            None => {}
        }
    }

    pub fn is_winner(&self) -> bool {
        // Horizontal
        for row in &self.positions {
            if self.check_winner_vec(row) { return true; }
        }

        // Vertical
        let mut idx: usize = 0;
        while idx < self.positions[0].len() {
            let column: Vec<Position> = self.positions
                .iter()
                .map(|col| col[idx])
                .collect();
            if self.check_winner_vec(&column) { return true; }
            idx += 1;
        }

        false
    }

    fn check_winner_vec(&self, row: &Vec<Position>) -> bool {
        let row_size = row.len();
        let marked: Vec<bool> = row
            .iter()
            .filter(|r| r.marked == true)
            .map(|r| r.marked)
            .collect();
        
        if marked.len() == row_size {
            return true;
        }

        false
    }
}

pub struct Solution {
    numbers: Vec<i32>,
    boards: Vec<Board>
}

impl Solution {
    pub fn new() -> Self {
        let mut sol = Self { 
            numbers: Vec::new(),
            boards: Vec::new(),
        };
        sol.process_input("day04/input.txt");

        sol
    }

    fn mark_boards(&mut self, value: &i32) {
        for board in self.boards.iter_mut() {
            board.mark(value);
        }
    }

    fn check_winners(&self) -> Option<(usize, &Board)> {
        for (pos, board) in self.boards.iter().enumerate() {
            if board.is_winner() {
                return Some((pos, &board));
            }
        }

        None
    }
}

impl Solve for Solution {
    fn process_input(&mut self, filepath: &str) {
        let input_data = read_file(filepath);
        let mut split: Vec<&str> = input_data
            .split("\n\n")
            .collect();

        let numbers: Vec<i32> = split.remove(0)
            .split(",")
            .map(|s| s.parse::<i32>().unwrap())
            .collect();

        let mut boards: Vec<Board> = Vec::new();
        for item in split.drain(..) {
            boards.push(Board::new(item));
        }

        self.numbers = numbers;
        self.boards = boards;
    }

    fn part1(&mut self) {
        let mut numbers = self.numbers.clone();
        for (pos, number) in numbers.drain(..).enumerate() {
            self.mark_boards(&number);

            // No wins before 4 goes
            if pos > 3 {
                if let Some(winner) = self.check_winners() {
                    let (_, winner) = winner;
                    let mut unmarked: Vec<i32> = Vec::new();
                    for (value, coord) in &winner.idx {
                        if winner.positions[coord.x][coord.y].marked == false {
                            unmarked.push(*value);
                        }
                    }
                    let sum: i32 = unmarked.iter().sum();
                    println!("Part 1: {}", sum * number);
                    break;
                }
            }
        }
    }

    fn part2(&mut self) {
        let mut numbers = self.numbers.clone();

        for (pos, number) in numbers.drain(..).enumerate() {
            self.mark_boards(&number);
            if pos < 4 { continue ;}
            
            let mut have_winner = true;
            let mut last_winner: Option<Board> = None;

            while have_winner {
                let winner = self.check_winners();
                match winner {
                    Some(winner) => {
                        let (pos, _) = winner;
                        last_winner = Some(self.boards.remove(pos));
                    }
                    None => { have_winner = false; }
                }            
            }

            if self.boards.is_empty() {
                let winner = last_winner.unwrap();
                let mut unmarked: Vec<i32> = Vec::new();
                for (value, coord) in &winner.idx {
                    if winner.positions[coord.x][coord.y].marked == false {
                        unmarked.push(*value);
                    }
                }

                let sum: i32 = unmarked.iter().sum();
                println!("Part 2: {}", sum * number);
                break;
            }
        }
    }
}