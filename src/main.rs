use std::time::Instant;
use std::time::Duration;
use crate::puzzle::AoCPuzzle;

mod puzzle;

mod day1;
mod day2;
mod day3;
mod day4;


fn print_results(name: &str, res: i64, time : Duration) {
    println!("{}:", name);
    println!("Result: {}", res);
    println!("Time: {}ms", time.as_millis());
}


fn run_and_print(puzzle: Box<dyn AoCPuzzle>) {

    let start = Instant::now();
    let res : i64 = puzzle.first_puzzle();
    let elapsed = start.elapsed();

    println!("------------------------------------------------------");
    print_results(&puzzle.puzzle_name(), res, elapsed);

    let start = Instant::now();
    let result : Option<i64> = puzzle.second_puzzle();
    let elapsed = start.elapsed();

    match result {
        Some(x) => {
            println!();
            print_results(&puzzle.puzzle_name(), x, elapsed);
        },
        None => {}
    }
    println!("------------------------------------------------------");


}


fn main() {
    println!("***************************************");
    println!("Advent of Code 2025 ..... in Rust");
    println!("***************************************");
    println!();
    println!("In order to learn some Rust features and idioms I have overengineered it (and made it worse) using some traits/structs, etc.");
    println!();

    let mut puzzles : Vec<Box<dyn AoCPuzzle>> = Vec::new();

    puzzles.push(Box::new(day1::Day1{filename : "resources/day1.txt".to_string()}));
    puzzles.push(Box::new(day2::Day2{filename : "resources/day2.txt".to_string()}));
    puzzles.push(Box::new(day3::Day3{filename : "resources/day3.txt".to_string()}));
    puzzles.push(Box::new(day4::Day4{filename : "resources/day4.txt".to_string()}));

    for puzzle in puzzles {
        run_and_print(puzzle);
    }



}
