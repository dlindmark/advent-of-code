use utils::Answer;

mod day1;
mod day2;

fn main() {
    let a1: Answer = day1::solve("day1.txt".to_string());
    println!("Day1: {:?}", a1);

    let a2: Answer = day2::solve("day2.txt".to_string());
    println!("Day2: {:?}", a2);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn day1() {
        let a: Answer = day1::solve("day1.txt".to_string());
        assert_eq!(a.challenge1, 70369);
        assert_eq!(a.challenge2, 203002);
    }
    #[test]
    fn day2() {
        let a: Answer = day2::solve("day2.txt".to_string());
        assert_eq!(a.challenge1, 12740);
        assert_eq!(a.challenge2, 11980);
    }
}