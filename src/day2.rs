use std::fs::read_to_string;
use crate::puzzle::AoCPuzzle;


// This one is 350ms
/*
fn is_form_xx(n: i64) -> Option<i64> {
    let mut n = n;
    if n < 0 {
        n = -n; // optional: handle negative numbers
    }

    // Count digits
    let mut len = 0;
    let mut temp = n;
    while temp > 0 {
        len += 1;
        temp /= 10;
    }

    if len % 2 != 0 || len == 0 {
        return None; // must have even number of digits
    }

    let half = len / 2;
    let div = 10_i64.pow(half as u32);
    let first = n / div;
    let second = n % div;

    if first == second {
        return Some(n)
    }
    
    None
} 

// this one is 330 ms
fn is_valid(n : i64) -> Option<i64> {
    let str = format!("{}", n);
    
    if str.len() % 2 == 0 {
        if str[0..str.len()/2] == str[str.len() / 2..str.len()] {
            return Some(str.parse::<i64>().unwrap());
        }
    }
    None
}

*/


/*
* Asymptotically, the most complex functions here are pow and log. Both of which should
* be in O(n log n)
*/
fn is_valid(n: i64) -> Option<i64> {

    let len = (n as f64).log10() as usize + 1;
    if len % 2 != 0 { return None; }

    let half = len / 2;
    let div = 10_i64.pow(half as u32);

    let left = n / div;
    let right = n % div;

    if left == right {
        return Some(n)
    }
    
    None
}


fn count_invalid_ids(start : i64, end : i64) -> i64 {
    let mut sum :i64 = 0;

    for i in start..=end {
        match is_valid(i) {
            Some(n) => sum += n,
            None => continue,
        }
    }

    sum
}


pub struct Day2 {
    
    pub filename: String
    
}

impl AoCPuzzle for Day2 {
    fn puzzle_name(&self) -> String {
        "Day 2".to_string()
    }

    fn first_puzzle(&self) -> i64 {
        let mut invalid_cnt : i64 = 0;

        for range in read_to_string(&self.filename).unwrap().trim().replace("\n","").split(",") {
            let tmp = range.split("-").collect::<Vec<&str>>();
            let start = tmp[0].parse::<i64>().unwrap();
            let end = tmp[1].parse::<i64>().unwrap();

            invalid_cnt += count_invalid_ids(start, end);
        }
        invalid_cnt
    }
    
    fn second_puzzle(&self) -> Option<i64> {
        None
    }
}