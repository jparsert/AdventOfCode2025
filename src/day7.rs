use crate::puzzle::AoCPuzzle;
use std::fs::read_to_string;


pub(crate) struct Day7 {
    pub filename: String,
}


impl AoCPuzzle for Day7 {
    fn puzzle_name(&self) -> String {
        "Day 7".to_owned()
    }

    fn first_puzzle(&self) -> i64 {
        let mut total = 0;
        let mut useless = 0;
        let mut matrix: Vec<Vec<char>> = Vec::new();

        for (i,line) in read_to_string(&self.filename).unwrap().lines().enumerate() {
            let mut vec: Vec<char> = line.chars().collect();

            if i > 0 {
                for j in 0..vec.len() {
                    match vec[j] {
                        '^' => {
                            total += 1;
                            match matrix[i-1][j] {
                                '.' => {
                                    useless += 1;
                                },
                                '|' => {
                                    vec[j - 1] = '|';
                                    vec[j + 1] = '|';
                                },
                                _ => ()
                            }
                        }
                        '.' => {
                            match matrix[i-1][j] {
                                'S' => {
                                    vec[j] = '|';
                                }
                                '|' => {
                                    vec[j] = '|'
                                },
                                _ => ()
                            }
                        }
                        _ => (),
                    }
                }
            }
            matrix.push(vec);
        }

        total - useless
    }

    fn second_puzzle(&self) -> Option<i64> {
        let mut matrix: Vec<Vec<char>> = Vec::new();
        let mut weights: Vec<i64> = Vec::new();


        for (i,line) in read_to_string(&self.filename).unwrap().lines().enumerate() {
            let mut vec: Vec<char> = line.chars().collect();

            if i == 0 {
                weights =  vec![0; vec.len()];
            }

            if i > 0 {
                for j in 0..vec.len() {
                    match vec[j] {
                        '^' => {
                            match matrix[i-1][j] {
                                '.' => {
                                },
                                '|' => {
                                    weights[j-1] += weights[j];
                                    weights[j+1] += weights[j];
                                    vec[j - 1] = '|';
                                    vec[j + 1] = '|';
                                },
                                _ => ()
                            }
                            weights[j] = 0;
                        }
                        '.' => {
                            match matrix[i-1][j] {
                                'S' => {
                                    vec[j] = '|';
                                    weights[j] = 1
                                }
                                '|' => {
                                    vec[j] = '|'
                                },
                                _ => ()
                            }
                        }
                        _ => (),
                    }
                }
            }
            matrix.push(vec);
        }

        Some(weights.iter().sum())

    }
}