use utils::Answer;

mod day1;
mod day2;
mod day3;
mod day4;
mod day5;

fn main() {
    let a1: Answer<i64> = day1::solve("day1.txt".to_string());
    println!("Day1: {:?}", a1);

    let a2: Answer<i64> = day2::solve("day2.txt".to_string());
    println!("Day2: {:?}", a2);

    let a3: Answer<i64> = day3::solve("day3.txt".to_string());
    println!("Day3: {:?}", a3);
    
    let a4: Answer<i64> = day4::solve("day4.txt".to_string());
    println!("Day4: {:?}", a4);

    let a5: Answer<String> = day5::solve("day5.txt".to_string());
    println!("Day5: {:?}", a5);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn day1() {
        let a: Answer<i64> = day1::solve("day1.txt".to_string());
        assert_eq!(a.challenge1.unwrap(), 70369);
        assert_eq!(a.challenge2.unwrap(), 203002);
    }
    #[test]
    fn day2() {
        let a: Answer<i64> = day2::solve("day2.txt".to_string());
        assert_eq!(a.challenge1.unwrap(), 12740);
        assert_eq!(a.challenge2.unwrap(), 11980);
    }

    #[test]
    fn day3() {
        let a: Answer<i64> = day3::solve("day3.txt".to_string());
        assert_eq!(a.challenge1.unwrap(), 8240);
        assert_eq!(a.challenge2.unwrap(), 2587);
    }
    
    #[test]
    fn day4() {
        let a: Answer<i64> = day4::solve("day4.txt".to_string());
        assert_eq!(a.challenge1.unwrap(), 528);
        assert_eq!(a.challenge2.unwrap(), 881);
    }

    #[test]
    fn day5() {
        let a: Answer<String> = day5::solve("day5.txt".to_string());
        assert_eq!(a.challenge1.unwrap(), String::from("RLFNRTNFB"));
        assert_eq!(a.challenge2.unwrap(), String::from("MHQTLJRLB"));
    }
}