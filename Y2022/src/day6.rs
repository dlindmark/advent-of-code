use utils::{collect_lines_in_file, Answer};
use std::collections::{VecDeque, HashSet};

pub fn solve(filename: String) -> Answer<usize> {
    let lines = collect_lines_in_file(filename);
    let marker = find_marker(&lines[0], 4);
    let marker2 = find_marker(&lines[0], 14);
    
    return Answer {challenge1: Some(marker), challenge2: Some(marker2)};
}

fn find_marker(line: &String, nb_distinct_chars: usize) -> usize {
    let mut ch_it = line.chars();
    let mut counter = 0;
    let mut four_ch: VecDeque<char> = Default::default();

    for _ in 0..nb_distinct_chars - 1{
        four_ch.push_back(ch_it.next().unwrap_or('\n').clone());
        counter += 1;
    }

    for ch in ch_it {
        counter += 1;
        four_ch.push_back(ch.clone());
        let mut uniq: HashSet<char> = HashSet::new();

        for i in 0..nb_distinct_chars{
            if !uniq.insert(four_ch[i]){
                break;
            }
        }
        if uniq.len() == four_ch.len(){
            break;
        }
        four_ch.pop_front();
    }

    return counter;
}


#[cfg(test)]
mod day6_tests {
    use super::{find_marker};

    #[test]
    fn test1(){
        let test: String = String::from("bvwbjplbgvbhsrlpgdmjqwftvncz");
        let marker = find_marker(&test, 4);
        assert_eq!(marker, 5);
    }

    #[test]
    fn test2(){
        let test: String = String::from("nppdvjthqldpwncqszvftbrmjlhg");
        let marker = find_marker(&test, 4);
        assert_eq!(marker, 6);
    }

    #[test]
    fn test3(){
        let test: String = String::from("nznrnfrfntjfmvfwmzdfjlvtqnbhcprsg");
        let marker = find_marker(&test, 4);
        assert_eq!(marker, 10);
    }

    #[test]
    fn test4(){
        let test: String = String::from("zcfzfwzzqfrljwzlrfnpqdbhtmscgvjw");
        let marker = find_marker(&test, 4);
        assert_eq!(marker, 11);
    }

    #[test]
    fn test5(){
        let test: String = String::from("mjqjpqmgbljsphdztnvjfqwrcgsmlb");
        let marker = find_marker(&test, 14);
        assert_eq!(marker, 19);
    }

    #[test]
    fn test6(){
        let test: String = String::from("nznrnfrfntjfmvfwmzdfjlvtqnbhcprsg");
        let marker = find_marker(&test, 14);
        assert_eq!(marker, 29);
    }
    
}