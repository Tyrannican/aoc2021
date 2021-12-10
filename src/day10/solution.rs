use super::{read_file, Solve};

fn score_illegal_char(ch: char) -> i32 {
    match ch {
        ')' => 3,
        ']' => 57,
        '}' => 1197,
        '>' => 25137,
        _ => 0
    }
}

#[derive(Debug)]
struct CharPair {
    opening: char,
    closing: char
}

pub struct Solution {
    samples: Vec<String>,
    corrupted_score: i32,
    incomplete_score: Vec<i64>
}

impl Solution {
    pub fn new() -> Self {
        let mut sol = Self {
            samples: Vec::new(),
            corrupted_score: 0,
            incomplete_score: Vec::new()
        };
        sol.process_input("day10/input.txt");

        sol
    }

    fn parse_sample(&mut self, sample: &String) {
        let mut stack = Vec::new();
        let mut corrupted = false;
        for char in sample.chars() {
            match char {
                '[' => { stack.push(CharPair { opening: '[', closing: ']' }) }
                '(' => { stack.push(CharPair { opening: '(', closing: ')' }) }
                '{' => { stack.push(CharPair { opening: '{', closing: '}' }) }
                '<' => { stack.push(CharPair { opening: '<', closing: '>' }) }
                ')' | '}' | '>'| ']' => {
                    corrupted = self.check_corrupted(&mut stack, char);
                    if corrupted { break; }
                }
                _ => {}
            }
        }

        // Not illegal, just incomplete
        if !corrupted {
            self.check_incomplete(&mut stack);
        }
    }

    fn check_corrupted(&mut self, stack: &mut Vec<CharPair>, ch: char) -> bool{
        let check = stack.pop().unwrap();
        if ch != check.closing {
            self.corrupted_score += score_illegal_char(ch);
            return true;
        }

        false
    }

    fn check_incomplete(&mut self, stack: &mut Vec<CharPair>) {
        let mut score = 0;
        while !stack.is_empty() {
            score *= 5;
            let pair = stack.pop().unwrap();

            match pair.closing {
                ')' => score += 1,
                ']' => score += 2,
                '}' => score += 3,
                '>' => score += 4,
                _ => {}
            }
        }

        self.incomplete_score.push(score);
    }
}

impl Solve for Solution {
    fn process_input(&mut self, filepath: &str) {
        let raw = read_file(filepath);

        self.samples = raw
            .split("\n")
            .map(|s| s.to_owned())
            .collect();
    }

    fn part1(&mut self) {
        for sample in &self.samples.clone() {
            self.parse_sample(sample);
        }
        
        self.incomplete_score.sort();
        let idx = self.incomplete_score.len() / 2;
        println!("Part 1: {}", self.corrupted_score);
        println!("Part 2: {}", self.incomplete_score[idx]);
    }

    fn part2(&mut self) {
        // Part 2 not needed as this solution does it all in part 1.
    }
}