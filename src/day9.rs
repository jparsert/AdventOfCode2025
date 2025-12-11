use std::fs::read_to_string;
use crate::puzzle::AoCPuzzle;

pub struct Day9 {
    pub filename: String,
}

fn component_dist(x : (i64, i64), y : (i64, i64)) -> (i64, i64) {
    ((x.0 - y.0).abs(), (x.1 - y.1).abs())
}

impl AoCPuzzle for Day9 {
    fn puzzle_name(&self) -> String {
        "Day 09".to_string()
    }


    fn first_puzzle(&self) -> i64 {
        let mut max = 0;

        let mut coordinate = Vec::new();
        for line in read_to_string(&self.filename).unwrap().lines() {
            let els : Vec<i64> = line.split(",").map(|x| x.parse::<i64>().unwrap()).collect();
            coordinate.push((els[0], els[1]));
        }

        for i in 0..coordinate.len() {
            for j in i + 1..coordinate.len() {
                let (x,y) = component_dist(coordinate[i], coordinate[j]);
                if x * y > max {
                    max = (x+1) * (y+1);
                }
            }
        }

        max
    }

    fn second_puzzle(&self) -> Option<i64> {
        None
    }
}
