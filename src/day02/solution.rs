use super::{read_file, Solve};

#[derive(PartialEq, Copy, Clone)]
enum Direction {
    Forward,
    Up,
    Down,
    Invalid
}

struct Movement {
    direction: Direction,
    distance: i32
}

impl Movement {
    pub fn new(direction: &str) -> Self {
        let split: Vec<&str> = direction.split(" ").collect();
        let direction = match split[0] {
            "forward" => Direction::Forward,
            "up" => Direction::Up,
            "down" => Direction::Down,
            _ => Direction::Invalid
        };

        let distance = split[1].parse::<i32>().unwrap();

        Self {
            direction,
            distance
        }
    }
}

pub struct Solution {
    course: Vec<Movement>
}

impl Solution {
    pub fn new() -> Self {
        let mut sol = Self { course: Vec::new() };
        sol.process_input("day02/input.txt");

        sol
    }
}


impl Solve for Solution {
    fn process_input(&mut self, filepath: &str) {
        let raw = read_file(filepath);
        let movement: Vec<&str> = raw.split("\n").collect();
        let mut moves: Vec<Movement> = Vec::new();
    
        for mv in movement {
            moves.push(Movement::new(mv));
        }
    
        self.course = moves
    }
    
    fn part1(&mut self) {
        let mut horz = 0;
        let mut depth = 0;
    
        for moveset in &self.course {
            let distance = moveset.distance;
            match moveset.direction {
                Direction::Down => depth += distance,
                Direction::Up => depth -= distance,
                Direction::Forward => horz += distance,
                _ => {}
            }
        }
    
        println!("Part 1: {}", horz * depth);
    }
    
    fn part2(&mut self) {
        let mut horz = 0;
        let mut depth = 0;
        let mut aim = 0;
    
        for moveset in &self.course {
            let distance = moveset.distance;
            match moveset.direction {
                Direction::Down => {
                    aim += distance;
                }
                Direction::Up => {
                    aim -= distance;
                }
                Direction::Forward => {
                    horz += distance;
                    
                    if aim != 0 {
                        let new_depth = aim * distance;
                        depth += new_depth;
                    }
                }
                _ => {}
            }
        }
    
        println!("Part 2: {}", horz * depth);
    }
}
