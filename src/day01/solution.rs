use super::read_file;
use super::Solve;

pub struct Solution {
    depth: Vec<i32>
}

impl Solution {
    pub fn new() -> Self {
        let mut sol = Self { depth: Vec::new() };
        sol.process_input("day01/input.txt");

        sol
    }
}

impl Solve for Solution {
    fn process_input(&mut self, fp: &str) {
        let raw = read_file(fp);
        self.depth = raw.split("\n")
            .into_iter()
            .map(|x| x.parse::<i32>().unwrap())
            .collect()
    }
    
    fn part1(&mut self) {
        let mut idx = 1;
        let mut increases = 0;
        while idx < self.depth.len() {
            if self.depth[idx] > self.depth[idx - 1] { increases += 1; }
            idx += 1;
        }
    
        println!("Solution 1: {}", increases);
    }
    
    fn part2(&mut self) {
        let mut sum = self.depth[0] + self.depth[1] + self.depth[2];
        let mut increases = 0;
        let mut idx = 1;
    
        while (idx + 2) < self.depth.len() {
            let window = self.depth[idx] + self.depth[idx + 1] + self.depth[idx + 2];
            if window > sum {
                increases += 1;
            }
    
            sum = window;
            idx += 1;
        }
    
        println!("Solution 2: {}", increases);
    }
}