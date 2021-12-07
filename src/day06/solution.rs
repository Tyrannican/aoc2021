use super::{read_file, Solve};

pub struct Solution {
    army: Vec<i128>
}

impl Solution {
    pub fn new() -> Self {
        let mut sol = Self { army: vec![0 as i128; 9] };
        sol.process_input("day06/input.txt");

        sol
    }

    fn tick(&mut self) {
        if self.army[0] != 0 {
            let zero_value = self.army[0];
            self.army[0] = 0;
            self.army.rotate_left(1);
            self.army[6] += zero_value;
            self.army[8] += zero_value;
        } else {
            self.army.rotate_left(1);
        }
    }
}

impl Solve for Solution {
    fn process_input(&mut self, filepath: &str) {
        let input = read_file(filepath);
        let days: Vec<i32> = input
            .split(",")
            .map(|f| f.parse::<i32>().unwrap())
            .collect();

            
        for day in days {
            self.army[day as usize] += 1;
        }
    }

    fn part1(&mut self) {
        for _ in 0..80 {
            self.tick();
        }

        println!("Part 1: {}", self.army.iter().sum::<i128>());
    }

    fn part2(&mut self) {
        for _ in 0..256 {
            self.tick();
        }

        println!("Part 2: {}", self.army.iter().sum::<i128>());
    }
}