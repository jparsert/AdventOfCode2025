use std::time::Instant;
use std::time::Duration;
use crate::puzzle::AoCPuzzle;

mod puzzle;

mod day1;
mod day2;
mod day3;
mod day4;
mod day5;
mod day6;
mod day7;
mod day8;
mod day9;
mod day10;
mod day11;
mod day12;

fn print_results(name: String, res: i64, time : Duration) {
    println!("{}:", name);
    println!("Result: {}", res);
    println!("Time: {}ms", time.as_millis());
}


fn run_and_print(puzzle: Box<dyn AoCPuzzle>) {

    let start = Instant::now();
    let res : i64 = puzzle.first_puzzle();
    let elapsed = start.elapsed();

    println!("------------------------------------------------------");
    print_results(puzzle.puzzle_name(), res, elapsed);

    let start = Instant::now();
    let result : Option<i64> = puzzle.second_puzzle();
    let elapsed = start.elapsed();

    match result {
        Some(x) => {
            println!();
            print_results(format!("{} Part 2",&puzzle.puzzle_name()), x, elapsed);
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


    /*
    puzzles.push(Box::new(day1::Day1{filename : "resources/day1.txt".to_string()}));
    puzzles.push(Box::new(day2::Day2{filename : "resources/day2.txt".to_string()}));
    puzzles.push(Box::new(day3::Day3{filename : "resources/day3.txt".to_string()}));
    puzzles.push(Box::new(day4::Day4{filename : "resources/day4.txt".to_string()}));
    puzzles.push(Box::new(day4::Day4Improved{filename : "resources/day4.txt".to_string()}));
    puzzles.push(Box::new(day5::Day5{filename : "resources/day5.txt".to_string()}));
    puzzles.push(Box::new(day6::Day6{filename : "resources/day6.txt".to_string()}));
    puzzles.push(Box::new(day7::Day7{filename : "resources/day7.txt".to_string()}));
    puzzles.push(Box::new(day8::Day8{filename : "resources/day8.txt".to_string()}));
    puzzles.push(Box::new(day9::Day9{filename : "resources/day9.txt".to_string()}));
    */
    //puzzles.push(Box::new(day10::Day10{filename : "resources/day10.txt".to_string()}));
    //puzzles.push(Box::new(day11::Day11{filename : "resources/day11.txt".to_string()}));
    puzzles.push(Box::new(day12::Day12{filename : "resources/day12.txt".to_string()}));


    for puzzle in puzzles {
        run_and_print(puzzle);
    }



}
