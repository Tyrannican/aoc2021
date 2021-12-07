use super::{read_file, Solve};
use std::collections::{HashSet, HashMap};

#[derive(Debug, Clone, Copy, PartialEq)]
struct Point {
    x: i32,
    y: i32
}

impl Point {
    pub fn from_str(input: &str) -> Self {
        let coords: Vec<i32> = input
            .split(",")
            .map(|p| p.parse::<i32>().unwrap())
            .collect();

        Self { x: coords[0], y: coords[1] }
    }
}

impl std::fmt::Display for Point {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "({},{})", self.x, self.y)
    }
}

#[derive(Debug, Copy, Clone, PartialEq)]
pub struct Line {
    start: Point,
    end: Point
}

impl Line {
    pub fn new(input: &String) -> Self {
        let coords: Vec<&str> = input.split(" -> ").collect();
        Self { 
            start: Point::from_str(coords[0]),
            end: Point::from_str(coords[1])
        }
    }

    fn slope(&self) -> Option<i32> {
        let y = self.end.y - self.start.y;
        let x = self.end.x - self.start.x;

        if y == 0 || x == 0 { return None; }

        Some(y / x)
    }

    fn generate_cardinal_range(&self) -> Vec<Point> {
        let mut points: Vec<Point> = Vec::new();

        if self.start.x == self.end.x {
            if self.end.y <= self.start.y {
                for i in self.end.y ..= self.start.y {
                    points.push(Point{ x: self.start.x, y: i });
                }
            } else {
                for i in self.start.y ..= self.end.y {
                    points.push(Point{ x: self.start.x, y: i });
                }
            }
        } else if self.start.y == self.end.y {
            if self.end.x <= self.start.x {
                for i in self.end.x ..= self.start.x {
                    points.push(Point{ x: i, y: self.start.y });
                }
            } else {
                for i in self.start.x ..= self.end.x {
                    points.push(Point{ x: i, y: self.start.y });
                }
            }
        }

        points
    }

    fn generate_full_range(&self) -> Vec<Point> {
        let mut points = Vec::new();

        let mut inc_x = 0;
        let mut inc_y = 0;
        let mut x = self.start.x;
        let mut y = self.start.y;

        if self.start.x < self.end.x {
            inc_x = 1;
        } else if self.start.x > self.end.x {
            inc_x = -1;
        }

        if self.start.y < self.end.y {
            inc_y = 1;
        } else if self.start.y > self.end.y {
            inc_y = -1;
        }

        while (x, y) != (self.end.x, self.end.y) {
            points.push(Point{ x, y });
            x += inc_x;
            y += inc_y;
        }

        points.push(Point{ x, y });

        points
    }

    fn generate_range(&self) -> Vec<Point> {
        match self.slope() {
            Some(_) => { self.generate_full_range() }
            None => { self.generate_cardinal_range() }
        }
    }
}

impl std::fmt::Display for Line {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "({},{}) <-> ({},{})", self.start.x, self.start.y, self.end.x, self.end.y)
    }
}

pub struct Solution {
    lines: Vec<Line>
}

impl Solution {
    pub fn new() -> Self {
        let mut sol = Self { lines: Vec::new() };
        sol.process_input("day05/input.txt");

        sol
    }
}

impl Solve for Solution {
    fn process_input(&mut self, filepath: &str) {
        let mut lines: Vec<Line> = Vec::new();
        let raw: Vec<String> = read_file(filepath)
            .split("\n")
            .map(|s| s.to_owned())
            .collect();

        for line in raw.iter() {
            lines.push(Line::new(line));
        }

        self.lines = lines;
    }

    fn part1(&mut self) {
        let straights: Vec<&Line> = self.lines
            .iter()
            .filter(|l| l.start.x == l.end.x || l.start.y == l.end.y)
            .collect();

        let mut map: HashMap<(i32, i32), i32> = HashMap::new();
        for line in &straights {
            let range = line.generate_range();
            for item in range {
                *map.entry((item.x, item.y)).or_insert(0) += 1;
            }
        }

        let counts: Vec<_> = map.values().filter(|v| **v > 1).collect();
        println!("Part 1: {}", counts.len());
    }

    fn part2(&mut self) {
        let mut map: HashMap<(i32, i32), i32> = HashMap::new();
        for line in &self.lines {
            let range = line.generate_range();
            for item in range  {
                *map.entry((item.x, item.y)).or_insert(0) += 1;
            }
        }

        let counts: Vec<_> = map.values().filter(|v|  **v > 1).collect();
        println!("Part 2: {}", counts.len());
    }
}
