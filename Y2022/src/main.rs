use utils::Answer;

mod day1;
mod day2;
mod day3;

fn main() {
    let a1: Answer = day1::solve("day1.txt".to_string());
    println!("Day1: {:?}", a1);

    let a2: Answer = day2::solve("day2.txt".to_string());
    println!("Day2: {:?}", a2);

    let a3: Answer = day3::solve("day3.txt".to_string());
    println!("Day3: {:?}", a3);
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

    #[test]
    fn day3() {
        let a: Answer = day3::solve("day3.txt".to_string());
        assert_eq!(a.challenge1, 8240);
        assert_eq!(a.challenge2, 2587);
    }
}