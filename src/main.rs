mod common;
use common::Solve;
mod day07;
use day07::solution::Solution;

fn main() {
    let mut s = Solution::new();
    s.part1();
    s.part2();
}
