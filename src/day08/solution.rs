use super::{read_file, Solve};
use std::collections::{HashMap, HashSet};


pub struct Solution {
    input_values: Vec<String>,
    output_values: Vec<String>
}

impl Solution {
    pub fn new() -> Self {
        let mut sol = Self {
            input_values: Vec::new(),
            output_values: Vec::new()
        };
        sol.process_input("day08/input.txt");

        sol
    }

    fn process_phrase(&mut self, digits: &Vec<&str>) -> HashMap<String, i32> {
        let mut map: HashMap<i32, String> = HashMap::new();
        let hs = HashSet::from(['a', 'b', 'c', 'd', 'e', 'f', 'g']);
        let mut known: HashSet<char> = HashSet::new();

        self.update_map(digits, &mut map, &mut known);
        let bottom_left: Vec<&char> = hs.difference(&known).collect();
        let eight: Vec<&str> = digits.iter().filter(|s| s.len() == 7).map(|s| *s).collect();
        let fives: Vec<&str> = digits.iter().filter(|s| s.len() == 5).map(|s| *s).collect();
        let sixes: Vec<&str> = digits.iter().filter(|s| s.len() == 6).map(|s| *s).collect();
        let one :Vec<String> = map.get(&1).unwrap().chars().map(|s| s.to_string()).collect();
        let four: Vec<String> = map.get(&4).unwrap().chars().map(|s| s.to_string()).collect();

        // 2,3,5
        for digit in &fives {
            let entry = digit.to_string();
            if digit.contains(&bottom_left[0].to_string()) && digit.contains(&bottom_left[1].to_string()) {
                map.insert(2, entry);
            } else if digit.contains(&one[0]) && digit.contains(&one[1]) {
                map.insert(3, entry);
            } else {
                map.insert(5, entry);
            }
        }

        // 0,6,9
        // Check 6 for missing char form 1
        for digit in &sixes {
            // 6
            let entry = digit.to_string();
            if !digit.contains(&one[0]) || !digit.contains(&one[1]) {
                map.insert(6, entry.clone());
                continue;
            }

            // 0
            for char in &four {
                if !digit.contains(char) {
                    map.insert(0, entry.clone());
                    continue;
                }
            }
        }

        let mut decoded: HashMap<String, i32> = HashMap::new();
        for (k, v) in map {
            decoded.insert(v, k);
        }

        for digit in &sixes {
            if !decoded.contains_key(digit.to_owned()) {
                decoded.insert(digit.to_string(), 9);
            }
        }
        
        decoded.insert(eight[0].to_owned(), 8);

        return decoded
    }

    fn update_map(&self, digits: &Vec<&str>, map: &mut HashMap<i32, String>, hashset: &mut HashSet<char>) {
        for digit in digits {
            match digit.len() {
                2 => {
                    let _: Vec<_> = digit.chars().map(|c| hashset.insert(c)).collect();
                    map.insert(1, digit.to_string());
                }
                3 => { 
                    let _: Vec<_> = digit.chars().map(|c| hashset.insert(c)).collect();
                    map.insert(7, digit.to_string());
                }
                4 => {
                    let _: Vec<_> = digit.chars().map(|c| hashset.insert(c)).collect();
                    map.insert(4, digit.to_string());
                }
                _ => {}
            }
        }
    }

    fn decode_output(&self, decoded: &HashMap<String, i32>, output: &String) -> i32 {
        let mut str_digits: Vec<String> = Vec::with_capacity(4);
        for digit in output.split(" ") {
            for (key, value) in decoded {
                if digit.len() == key.len() {
                    let mut d_chars: Vec<char> = digit.chars().collect();
                    let mut k_chars: Vec<char> = key.chars().collect();
                    d_chars.sort_by(|a, b| b.cmp(a));
                    k_chars.sort_by(|a, b| b.cmp(a));
                    
                    if k_chars == d_chars {
                        str_digits.push(value.to_string());
                    }
                }
            }
        }

        let str_number: String = str_digits.into_iter().collect();
        str_number.parse::<i32>().unwrap()
    }
}

impl Solve for Solution {
    fn process_input(&mut self, filepath: &str) {
        let raw = read_file(filepath);
        let mut input_data: Vec<&str> = raw.split("\n").collect();

        for line in input_data.drain(..) {
            let split: Vec<&str> = line.split(" | ").collect();
            self.input_values.push(split[0].to_owned());
            self.output_values.push(split[1].to_owned());
        }
    }

    fn part1(&mut self) {
        let mut counter: usize = 0;
        for val in &self.output_values {
            counter += val
                .split(" ")
                .filter(|s| s.len() == 2 || s.len() == 3 || s.len() == 4 || s.len() == 7)
                .count();
        }

        println!("{}", counter);
    }

    fn part2(&mut self) {
        // 1 == 2 letters
        // 7 == 3 letters
        // 4 == 4 letters
        // 2,3,5 == 5 letters
        // 0,6,9 == 6 letters
        // 8 == 7 letters
        
        let inputs = self.input_values.clone();
        let outputs = self.output_values.clone();

        let mut total = 0;
        for (phrase, output) in inputs.iter().zip(outputs.iter()) {
            let digits: Vec<&str> = phrase.split(" ").collect();
            let decoded = self.process_phrase(&digits);
            total += self.decode_output(&decoded, output);
        }

        println!("Part 2: {}", total);
    }
}