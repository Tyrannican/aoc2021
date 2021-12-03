mod common;
use common::Solve;
mod day03;
use day03::solution::Solution;

fn main() {
    let mut s = Solution::new();
    s.part1();
    s.part2();
}
