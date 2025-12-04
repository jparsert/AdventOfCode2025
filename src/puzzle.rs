

pub trait AoCPuzzle {
    
    fn puzzle_name(&self)-> String;
    
    fn first_puzzle(&self) -> i64;
    
    fn second_puzzle(&self) -> Option<i64>;
    
}
