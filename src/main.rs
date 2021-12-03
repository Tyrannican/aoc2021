mod common;
use common::Solve;
mod day04;
use day04::solution::Solution;

fn main() {
    let mut s = Solution::new();
    s.part1();
    s.part2();
}
