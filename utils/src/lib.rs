use std::io;
use std::fs::File;
use std::io::{BufRead};
use std::path::Path;

#[derive(Debug)]
pub struct Answer {
    pub challenge1: i64,
    pub challenge2: i64
}



// impl Answer {
//     fn new(challenge1: i64, challenge2: i64) -> Answer{

//         Answer {challenge1: challenge1, challenge2: challenge2}
//     }
// }

// The output is wrapped in a  Result to allow matching on errors
// Returns an Iterator to the Reader of the lines of the file
pub fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where P: AsRef<Path>, {
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}


pub fn collect_lines_in_file(filename: String) -> Vec<String> {
    if let Ok(lines) = read_lines(filename){
        return lines.filter_map(|s| s.ok()).collect();
    }
    return vec![];
}