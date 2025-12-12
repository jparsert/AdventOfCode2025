use std::collections::{HashSet, VecDeque};
use std::fs::read_to_string;
use crate::puzzle::AoCPuzzle;


pub struct Day10 {
pub filename: String,
}

fn press_buttons(vec : &Vec<bool>, button: &Vec<usize>) -> Vec<bool> {
    let mut res = vec.clone();
    for b in button {
        res[*b] = res[*b] ^ true
    }
    res
}

fn get_minimal_presses(target : &Vec<bool>, start : Vec<bool>, buttons : &Vec<Vec<usize>>) -> Option<i32> {
    //BFS
    let mut queue : VecDeque<Vec<Vec<bool>>> = VecDeque::new();
    queue.push_front(vec![start]);
    let mut seen : HashSet<Vec<bool>> = HashSet::new();

    while !queue.is_empty() {
        let current = queue.pop_front().unwrap();
        let last = current.last().unwrap();
        if last.eq(target) {
            return Some((current.len() - 1) as i32);
        }
        seen.insert(last.clone());

        for button in buttons {
            let mut path = current.clone();
            let new = press_buttons(last, button);
            if !seen.contains(&new) {
                path.push(new);
                queue.push_back(path);
            }
        }
    }
    None
}

impl AoCPuzzle for Day10 {
    fn puzzle_name(&self) -> String {
        "Day 10".to_string()
    }

    fn first_puzzle(&self) -> i64 {
        let mut acc = 0;
        for line in read_to_string(&self.filename).unwrap().lines() {
            let split = line.split(" ").collect::<Vec<&str>>();
            let target = split[0][1..split[0].len()-1].chars().map(|c| c == '#').collect::<Vec<bool>>();
            let wiring: Vec<Vec<usize>> = split[1..split.len() - 1].iter().map(|x| x[1..x.len() - 1].split(",").map(|x| x.parse::<usize>().unwrap()).collect()).collect();

            acc += match get_minimal_presses(&target, vec![false; target.len()] ,&wiring) {
                Some(x) => x,
                None => panic!("Target could not be found."),
            }
        }

        acc as i64
    }

    fn second_puzzle(&self) -> Option<i64> {
        None
    }
}