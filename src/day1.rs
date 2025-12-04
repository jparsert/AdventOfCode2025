use std::fs::read_to_string;

use crate::puzzle;

/* *
fn wrap_cnt_mod(x: i32, delta : i32, max : i32) -> (i32, i32) {
    let tmp = max + 1;
    let tot = x + delta;

    let res = tot.rem_euclid(tmp);
    let wraps = tot.div_euclid(tmp);

    (res, wraps)
}
*/

pub struct Day1 {
    pub filename: String
}

fn wrap_mod(x: i32, max : i32) -> i32 {
    (x % (max + 1) + (max + 1)) % (max + 1)
}


impl puzzle::AoCPuzzle for Day1 {
    fn puzzle_name(&self) -> String {
        "Day 1".to_string()
    }

    fn first_puzzle(&self) -> i64 {
        let mut zero_cnt : i32 = 0;
        let mut current_dial : i32 = 50;

        for line in read_to_string(&self.filename).unwrap().lines() {
            let num : i32 = line[1..].parse::<i32>().unwrap();
            match &line[0..1] {
                "R" => current_dial = wrap_mod(current_dial + num, 99),
                "L" => current_dial = wrap_mod(current_dial - num, 99),
                _ => panic!("Unknown direction: {}", line),
            }

            if current_dial == 0 {
                zero_cnt += 1;
            }
        }

        zero_cnt as i64
    }

    fn second_puzzle(&self) -> Option<i64> {
        None
    }
}
