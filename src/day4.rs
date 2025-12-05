use std::fs::read_to_string;
use std::collections::{HashSet, VecDeque};
use crate::puzzle::AoCPuzzle;

fn load_matrix(input_file : &str) -> Vec<Vec<char>> {
    let mut mat :VecDeque<Vec<char>> = VecDeque::new();

    for line in read_to_string(input_file).unwrap().lines() {
        let mut v: Vec<char> = Vec::new();
        v.push('.');
        for char in line.trim().chars() {
            v.push(char);
        }
        v.push('.');
        mat.push_back(v);
    }

    mat.push_back(vec!['.'; mat[0].len()]);
    mat.push_front(vec!['.'; mat[0].len()]);

    mat.into()
}

fn load_hash_set(input_file : &str) -> HashSet<(i32, i32)> {
    let mut set :HashSet<(i32, i32)> = HashSet::new();

    for (i,line) in read_to_string(input_file).unwrap().lines().enumerate() {
        for (j,char) in line.trim().chars().enumerate() {
            if char == '@' {
                set.insert((i as i32, j as i32));
            }
        }
    }

    set
}

fn cnt_neighbours_hash_set(i : i32 ,j : i32 , set : &HashSet<(i32, i32)>) -> i32 {
    let mut cnt = 0;

    if set.contains(&(i - 1, j - 1)) { cnt +=1; }
    if set.contains(&(i - 1, j)) { cnt +=1; }
    if set.contains(&(i - 1, j+1)) { cnt +=1; }
    if set.contains(&(i, j+1)) { cnt +=1; }
    if set.contains(&(i+1, j+1)) { cnt +=1; }
    if set.contains(&(i+1, j)) { cnt +=1; }
    if set.contains(&(i+1, j-1)) { cnt +=1; }
    if set.contains(&(i, j-1)) { cnt +=1; }

    cnt
}


fn cnt_neighbours(i : usize ,j : usize , matrix : &Vec<Vec<char>>) -> i32 {
    let mut cnt = 0;

    if matrix[i-1][j-1] == '@' { cnt +=1; }
    if matrix[i-1][j] == '@' { cnt +=1; }
    if matrix[i-1][j+1] == '@' { cnt +=1; }
    if matrix[i][j+1] == '@' { cnt +=1; }
    if matrix[i+1][j+1] == '@' { cnt +=1; }
    if matrix[i+1][j] == '@' { cnt +=1; }
    if matrix[i+1][j-1] == '@' { cnt +=1; }
    if matrix[i][j-1] == '@' { cnt +=1; }

    return cnt;
}

/**
 Let's do the naive and slow solution first.
*/


pub struct Day4 {
    pub filename: String
}



impl AoCPuzzle for Day4 {
    fn puzzle_name(&self) -> String {
        "Day 4".to_string()
    }

    fn first_puzzle(&self) -> i64 {
        let mut cnt = 0;

        let matrix = load_matrix(&self.filename);
        for i in 1..matrix.len()-1 {
            for j in 1..matrix[i].len()-1 {
                if matrix[i][j] == '@' {
                    let num_neightbours = cnt_neighbours(i,j, &matrix);
                    if num_neightbours < 4 { cnt += 1 }
                }
            }
        }

        cnt as i64
    }

    fn second_puzzle(&self) -> Option<i64> {
        let mut cnt = 0;
        let mut change : bool = true;
        let mut matrix = load_matrix(&self.filename);

        while change {
            change = false;
            for i in 1..matrix.len() - 1 {
                for j in 1..matrix[i].len() - 1 {
                    if matrix[i][j] == '@' {
                        let num_neightbours = cnt_neighbours(i, j, &matrix);
                        if num_neightbours < 4 {
                            matrix[i][j] = '.';
                            change = true;
                            cnt += 1;
                        }
                    }
                }
            }
        }

        Some(cnt)
    }
}

pub struct Day4Improved {
    pub filename: String
}

impl AoCPuzzle for Day4Improved {
    fn puzzle_name(&self) -> String {
        "Day 4 With HashSet (which, surprisingly, turns out to be slower, my guss is cash hits with Vec are better). Fewer LoC and more elegant".to_string()
    }

    fn first_puzzle(&self) -> i64 {
        let hash_set = load_hash_set(&self.filename);
        let mut cnt = 0;

        for &(i,j) in &hash_set {
            if cnt_neighbours_hash_set(i,j, &hash_set) < 4 {
                cnt += 1;
            }
        }

        cnt
    }

    fn second_puzzle(&self) -> Option<i64> {
        None
    }
}
