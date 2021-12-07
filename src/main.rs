mod common;
use common::Solve;
mod day06;
use day06::solution::Solution;

fn main() {
    let mut s = Solution::new();
    s.part1();
    s.part2();
}
