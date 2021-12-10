mod common;
use common::Solve;
mod day10;
use day10::solution::Solution;

fn main() {
    let mut s = Solution::new();
    s.part1();
    s.part2();
}
