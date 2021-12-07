use super::{read_file, Solve};

fn coeff(num: i32) -> i32 {
    ((num * num) + num) / 2
}

pub struct Solution {
    positions: Vec<i32>
}

impl Solution {
    pub fn new() -> Self {
        let mut sol = Self {positions: Vec::new() };
        sol.process_input("day07/input.txt");

        sol
    }
}

impl Solve for Solution {
    fn process_input(&mut self, filepath: &str) {
        let input = read_file(filepath);

        self.positions = input
            .split(",")
            .map(|p| p.parse::<i32>().unwrap())
            .collect();
    }

    fn part1(&mut self) {
        let mut fuel_cost = i32::MAX;
        let mut cost = 0;
        
        for i in 0..self.positions.len() {
            cost = 0;
            for mv in &self.positions {
                cost += i32::abs(mv - i as i32);
            }

            if cost < fuel_cost {
                fuel_cost = cost;
            }
        }

        println!("Part 1: {}", fuel_cost);
    }

    fn part2(&mut self) {
        let mut fuel_cost = i32::MAX;
        let mut cost = 0;

        for i in 0..self.positions.len() {
            cost = 0;
            for mv in &self.positions {
                let og_cost = i32::abs(mv - i as i32);
                let total_cost = coeff(og_cost);
                cost += total_cost;
            }

            if cost < fuel_cost {
                fuel_cost = cost;
            }
        }

        println!("Part 2: {}", fuel_cost);
    }
}