use utils::Answer;

mod day1;
mod day2;

fn main() {
    let a1: Answer = day1::day1("day1.txt".to_string());
    println!("Day1: {:?}", a1);

    let a2: Answer = day2::solve("day2.txt".to_string());
    println!("Day2: {:?}", a2);
}
