use super::{read_file, Solve};

const DIRECTION: [(i32, i32); 8] = [(-1, 0), (-1, 1), (0, 1), (1, 1), (1, 0), (1, -1), (0, -1), (-1, -1)];

fn xy(map: &Vec<Vec<Position>>, x: usize, y: usize) -> i32 {
    let null = vec![];
    let row = map.get(x).unwrap_or(&null);
    let res = &*row.get(y).unwrap_or(&Position{value: -1, flashed: false});

    res.value
}

#[derive(Default)]
pub struct Position {
    value: i32,
    flashed: bool
}

pub struct Solution {
    map: Vec<Vec<Position>>,
    height: usize,
    width: usize,
    total_flashes: i32,
}

impl Solution {
    pub fn new() -> Self {
        let mut sol = Self {
            map: Vec::new(),
            height: 0,
            width: 0,
            total_flashes: 0,
        };
        sol.process_input("day11/input.txt");

        sol
    }

    fn increment(&mut self) {
        for x in 0..self.height {
            for y in 0..self.width {
                self.map[x][y].value += 1;
            }
        }
    }

    fn check_flash(&mut self) {
        loop {
            let mut flashes = 0;
            for x in 0..self.height {
                for y in 0..self.width {
                    if self.map[x][y].value > 9 && !self.map[x][y].flashed {
                        self.flash(x, y);
                        flashes += 1;
                    }
                }
            }

            // self.reset_flash();
            if flashes == 0 { break;}
        }
    }

    fn flash(&mut self, x: usize, y: usize) {
        self.map[x][y].value = 0;
        self.map[x][y].flashed = true;
        self.total_flashes += 1;

        for (dx, dy) in DIRECTION {
            let x = (x as i32 + dx) as usize;
            let y = (y as i32 + dy) as usize;
            if xy(&self.map, x, y) > -1 {
                self.map[x][y].value += 1;
                if self.map[x][y].value > 9 { self.flash(x, y); }
            }
        }
    }

    fn reset_flash(&mut self) {
        for x in 0..self.height {
            for y in 0..self.width {
                if self.map[x][y].flashed {
                    self.map[x][y].flashed = false;
                    self.map[x][y].value = 0;
                }
            }
        }
    }

    fn step(&mut self) {
        self.increment();
        self.check_flash();
        self.reset_flash();
    }
}

impl Solve for Solution {
    fn process_input(&mut self, filepath: &str) {
        let raw = read_file(filepath);

        let mut map: Vec<Vec<Position>> = Vec::new();
        for row in raw.split("\n") {
            let values: Vec<Position> = row
                .chars()
                .map(|c| {
                    Position{
                        value: c.to_string().parse::<i32>().unwrap(),
                        flashed: false
                    }
                })
                .collect();

            map.push(values);
        }

        self.map = map;
        self.height = self.map.len();
        self.width = self.map[0].len();
    }

    fn part1(&mut self) {
        for _ in 0..100 {
            self.step();
        }

        println!("Part 1: {}", self.total_flashes);
    }

    fn part2(&mut self) {
        let mut pt2 = Self::new();
        let mut step = 1;
        loop {
            let prev = pt2.total_flashes;
            pt2.step();
            let after = pt2.total_flashes;

            if (after - prev) == (self.height as i32 * self.width as i32) {
                println!("Step: {}", step);
                break;
            }
            step += 1;
        }

        println!("Part 2: {}", "EXIT");
    }
}