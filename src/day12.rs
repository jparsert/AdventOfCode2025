use std::fs::read_to_string;
use crate::puzzle::AoCPuzzle;

pub struct Day12 {
    pub filename: String,

}

/*
fn read_shape(string: String) -> (Vec<Vec<bool>>, i32) {
    let mut shape : Vec<Vec<bool>> = Vec::new();
    let mut size = 0;

    let x = string.split('\n').collect::<Vec<&str>>();
    for line in x[1..].iter() {
        shape.push(line.chars().map(|c| c == '#').collect());
        size += line.chars().filter(|x|*x == '#').count();
    }
    (shape,size as i32)
}
*/

fn parse_space(string:String) -> (i32, i32) {
    let binding = string.trim()[0..string.len()-1].to_string();
    let s = binding.split("x").collect::<Vec<_>>();

    let x = s[0].parse::<i32>().unwrap();
    let y = s[1].parse::<i32>().unwrap();
    (x, y)
}


impl AoCPuzzle for Day12 {
    fn puzzle_name(&self) -> String {
        "Day 12".to_string()
    }

    fn first_puzzle(&self) -> i64 {
        let num_shapes = 6;
        let mut cnt = 0;

        let lines : Vec<String> = read_to_string(&self.filename).unwrap().split("\n\n").map(|s| s.to_string()).collect();

        for line in lines[num_shapes].lines() {
            let split = line.split(" ").collect::<Vec<&str>>();
            let avail_space = parse_space(split[0].to_string());

            let overapproximation = split[1..].iter().map(|s| s.parse::<i32>().unwrap()).sum::<i32>() * 9;

            if avail_space.0 * avail_space.1 >= overapproximation {
                cnt += 1;
            }
        }

        cnt
    }

    fn second_puzzle(&self) -> Option<i64> {
        None
    }
}