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

    fn most_common_bit(&mut self, idx: usize) -> Bit {
        let size = self.data.len();

        let ones = self.data
        .iter()
        .filter(|x| x.chars().nth(idx).unwrap() == '1')
        .count();
        
        let zeros = size - ones;
        
        if ones > zeros || ones == zeros {
            return Bit::One;
        }
        
        Bit::Zero
    }

    pub fn get_rating(&mut self, common: bool) -> u32 {
        0
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
            let most_common = self.most_common_bit(i);
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
        self.data = vec![
            String::from("00100"),
            String::from("11110"),
            String::from("10110"),
            String::from("10111"),
            String::from("10101"),
            String::from("01111"),
            String::from("00111"),
            String::from("11100"),
            String::from("10000"),
            String::from("11001"),
            String::from("00010"),
            String::from("01010"),
        ];
    
        let o2_rating = self.get_rating(true);
        let co2_rating= self.get_rating(false);
    
        println!("O2: {}, CO2: {}", o2_rating, co2_rating);
        println!("Part 2: {}", o2_rating * co2_rating);
    }
}


