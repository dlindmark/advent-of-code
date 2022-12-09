use utils::{Answer, collect_lines_in_file};
use std::cmp::{min, max};
use std::collections::VecDeque;

pub fn solve(filename: String) -> Answer{
    
}


struct Instruction {
    nb: i64,
    from_stack: i64,
    to_stack: i64
}


fn parse_into_queues_and_instructions(filename: String) -> bool {
    let lines: Vec<String> = collect_lines_in_file(filename);
    
    

    return true;
}


// In how many assignment pairs does one range fully contain the other?
fn challenge1(ranges: &Vec<((i32, i32),(i32, i32))>) -> i64{

}


fn check_overlap(range1: (i32, i32), range2: (i32, i32)) -> i32 {

}


fn challenge2(ranges: &Vec<((i32, i32),(i32, i32))>) -> i64{

}


#[cfg(test)]
mod day4_tests {
    use super::check_overlap;

    #[test]
    fn first_range_smaller(){
        let o = check_overlap((2,3), (4,5));
        assert_eq!(o, 0);
    }

}