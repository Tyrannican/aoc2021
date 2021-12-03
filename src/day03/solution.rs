use super::{read_file, Solve};

#[derive(PartialEq, Copy, Clone)]
enum Bit {
    One,
    Zero
}

pub struct Solution {
    data: Vec<String>
}

impl Solution {
    pub fn new() -> Self {
        let mut sol = Self { data: Vec::new() };
        sol.process_input("day03/input.txt");

        sol
    }

    fn most_common_bit(&self, data: &Vec<String>, idx: usize) -> Bit {
        let size = data.len();

        let ones = data
        .iter()
        .filter(|x| x.chars().nth(idx).unwrap() == '1')
        .count();
        
        let zeros = size - ones;
        
        if ones > zeros || ones == zeros {
            return Bit::One;
        }
        
        Bit::Zero
    }

    fn filter_data(&self, data: &Vec<String>, idx: usize, pattern: char) -> Vec<String> {
        data
            .into_iter()
            .filter(|s| s.chars().nth(idx).unwrap() == pattern)
            .map(|s| s.to_owned())
            .collect()
    }

    pub fn get_rating(&mut self, common: bool) -> u32 {
        let mut idx: usize = 0;
        let mut rating: Vec<String> = self.data.clone();
        let item_size = rating[0].len();

        while rating.len() > 1 {
            if idx >= item_size {
                break
            }

            let most_common = self.most_common_bit(&rating, idx);
            match most_common {
                Bit::One => {
                    if common {
                        rating = self.filter_data(&rating, idx, '1')
                    } else {
                        rating = self.filter_data(&rating, idx, '0')
                    }
                }
                Bit::Zero => {
                    if common {
                        rating = self.filter_data(&rating, idx, '0')
                    } else {
                        rating = self.filter_data(&rating, idx, '1')
                    }
                }
            };

            idx += 1;
        }

        u32::from_str_radix(&rating[0], 2).unwrap()
    }
}

impl Solve for Solution {
    fn process_input(&mut self, filepath: &str) {
        let raw = read_file(filepath);
        self.data = raw.split("\n").map(|s| s.to_owned()).collect();
    }
    
    fn part1(&mut self) {
        let item_size = self.data[0].len();
        let mut gamma = String::from("");
        let mut epsilon = String::from("");
        
        for i in 0..item_size {
            let most_common = self.most_common_bit(&self.data, i);
            match most_common {
                Bit::One => {
                    gamma.push_str("1");
                    epsilon.push_str("0");
                }
                Bit::Zero => {
                    gamma.push_str("0");
                    epsilon.push_str("1");
                }
            }
        }
    
        let gamma_val = u32::from_str_radix(&gamma, 2).unwrap();
        let epsilon_val = u32::from_str_radix(&epsilon, 2).unwrap();
    
        println!("Part 1: {}", gamma_val * epsilon_val);
    }
    
    fn part2(&mut self) {    
        let o2_rating = self.get_rating(true);
        let co2_rating= self.get_rating(false);
    
        println!("Part 2: {}", o2_rating * co2_rating);
    }
}


