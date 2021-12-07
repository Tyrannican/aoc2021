mod common;
use common::Solve;
mod day05;
use day05::solution::Solution;

fn main() {
    let mut s = Solution::new();
    s.part1();
    s.part2();
}
