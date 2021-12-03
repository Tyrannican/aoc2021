use std::fs;
use std::env;

pub fn read_file(fp: &str) -> String {
    let mut path = env::current_dir().unwrap();
    path = path.join(&format!("src/{}", fp));
    let res = fs::read_to_string(path);
    match res {
        Ok(res) => { return res; }
        Err(e) => { panic!("Cannot open file {}\n{}", fp, e); }
    }
}

pub trait Solve {
    fn process_input(&mut self, filepath: &str);
    fn part1(&mut self);
    fn part2(&mut self);
}