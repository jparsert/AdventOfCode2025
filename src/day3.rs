use std::fs::read_to_string;
use crate::puzzle::AoCPuzzle;

/*
4k ms

fn maximum_joltage(bank : &str) -> i32 {
    let mut max = 0;
    for i in 0.. bank.len() {
        for j in i + 1..bank.len() {
            let val = format!("{}{}",bank.chars().nth(i).unwrap(), bank.chars().nth(j).unwrap());
            if val.parse::<i32>().unwrap() > max {
                max = val.parse::<i32>().unwrap();
            }
        }
    }

    max
}
*/

pub struct Day3 {
    
    pub filename : String

}


fn maximum_joltage(bank : &str) -> i32 {
    let mut max = 0;
    let bnk : Vec<i32> = bank.chars().map(|c| c.to_digit(10).unwrap() as i32).collect();

    for i in 0.. bank.len() {
        for j in i + 1..bank.len() {
            if 10 * bnk[i] + bnk[j] > max {
                max =  10 * bnk[i] + bnk[j];
            }
        }
    }

    max
}

impl AoCPuzzle for Day3 {
    fn puzzle_name(&self) -> String {
        "Day 3".to_string()
    }

    fn first_puzzle(&self) -> i64 {
        let res : i32 = read_to_string(&self.filename).unwrap().lines().map(maximum_joltage).sum();
        res as i64
    }

    fn second_puzzle(&self) -> Option<i64> {
        None
    }
}
