use std::fmt::Debug;

use super::{read_file, Solve};

use std::collections::HashSet;

#[derive(Debug)]
struct Point {
    x: usize,
    y: usize,
    value: i32
}

#[derive(Debug, Default)]
pub struct Map {
    points: Vec<Vec<i32>>,
    low_points: Vec<Point>
}

impl Map {
    fn new(mappings: &str) -> Self {
        let mut points = Vec::new();

        for row in mappings.split("\n") {
            let pts: Vec<i32> = row
                .chars()
                .map(|c|  c.to_string().parse::<i32>().unwrap())
                .collect();
            points.push(pts);
        }

        Self { points, low_points: Vec::new() }
    }

    fn calculate_low_points(&mut self) {
        let height = self.points.len();
        let width = self.points[0].len();
        let direction = vec![(1, 0), (0, -1), (0, 1), (-1, 0)];

        for x in 0..height {
            for y in 0..width {
                let value = self.points[x][y];
                let minimum: i32 = direction
                    .iter()
                    .map(|(dir_x, dir_y)| self.xy(x, y, *dir_x, *dir_y))
                    .min()
                    .unwrap();
                if value < minimum {
                    self.low_points.push(Point{ x, y, value });
                }
            }
        }
    }

    fn calculate_basins(&self) -> i32 {
        let mut totals = Vec::new();
        for point in &self.low_points {
            let surrounding = self.get_surrounding_points(point.x, point.y);
            totals.push(surrounding.len() as i32);
        }
        
        totals.sort();
        totals.reverse();
        totals[0] * totals[1] * totals[2]
    }
    
    fn xy(&self, x: usize, y: usize, dir_x: i32, dir_y: i32) -> i32 {
        let null = vec![];
        let x = x as i32 + dir_x;
        let y = y as i32 + dir_y;
        
        let row = self.points.get(x as usize).unwrap_or(&null);
        *row.get(y as usize).unwrap_or(&10)
    }
    
    fn get_surrounding_points(&self, x: usize, y: usize) -> Vec<(usize, usize)> {
        // let directions = vec![(-1, -1), (-1, 0), (-1, 1), (0, -1), (0, 1), (1, -1), (1, 0), (1, 1)];
        let directions = vec![(-1, 0), (0, -1), (0, 1), (1, 0)];
        let mut points = vec![(x, y)];
        let mut hs = HashSet::new();
        loop {
            let mut to_add = Vec::new();
            for (pt_x, pt_y) in points.drain(..) {
                for (dx, dy) in &directions {
                    if self.xy(pt_x, pt_y, *dx, *dy) < 9 {
                        let new_x = pt_x as i32 + dx;
                        let new_y = pt_y as i32 + dy;
                        to_add.push((new_x, new_y));
                    }
                }
                
            }
            
            for (x, y) in to_add {
                if !hs.contains(&(x, y)) {
                    hs.insert((x, y));
                    points.push((x as usize, y as usize));
                }
            }

            if points.is_empty() { break; }
        }

        hs.into_iter().map(|(x, y)| (x as usize, y as usize)).collect::<Vec<(usize, usize)>>()
    }
}

pub struct Solution {
    map: Map
}

impl Solution {
    pub fn new() -> Self {
        let mut sol = Self { map: Map::default() };
        sol.process_input("day09/input.txt");

        sol
    }
}

impl Solve for Solution {
    fn process_input(&mut self, filepath: &str) {
        let raw = read_file(filepath);
        self.map = Map::new(&raw);
    }

    fn part1(&mut self) {
        self.map.calculate_low_points();
        let mut sum: i32 = self.map.low_points
            .iter()
            .map(|l| l.value)
            .sum();
        sum += self.map.low_points.len() as i32;
        println!("Part 1: {}", sum);
    }

    fn part2(&mut self) {
        let largest_basins = self.map.calculate_basins();
        println!("Part 2: {}", largest_basins);
    }
}