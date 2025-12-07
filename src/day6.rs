use crate::puzzle::AoCPuzzle;
use std::fs::read_to_string;

pub struct Day6 {
    pub filename : String,
}


impl AoCPuzzle for Day6 {
    fn puzzle_name(&self) -> String {
        "Day 6".to_string()
    }

    fn first_puzzle(&self) -> i64 {
        let mut total : i64 = 0;
        let mut vals : Vec<Vec<i64>> = Vec::new();

        for line in read_to_string(&self.filename).unwrap().lines() {
            let els = line.split_whitespace().collect::<Vec<&str>>();
            if els.first().unwrap().parse::<i64>().is_ok() {
                vals.push(els.iter().map(|s| s.parse::<i64>().unwrap()).collect());
            } else {
                for (i, op) in els.iter().enumerate() {
                    match op.chars().next().unwrap() {
                       '+' => total += vals.iter().map(|vec| vec[i]).sum::<i64>(),
                        '*' => total += vals.iter().map(|vec| vec[i]).fold(1, |a, b| a * b),
                        _ => unreachable!()
                    }
                }
            }
        }

        total
    }

    fn second_puzzle(&self) -> Option<i64> {
        None
    }
}