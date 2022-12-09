use utils::{Answer, collect_lines_in_file};
use std::cmp::{min, max};

pub fn solve(filename: String) -> Answer<i64>{
    let lines: Vec<String> = collect_lines_in_file(filename);
    // println!("{}", lines[0].split(","));
    let lines2: Vec<Vec<String>> = lines.into_iter()
                                .map(|s: String| s.split(['-', ','].as_ref())
                                    .map(|s: &str| s.to_string()).collect()
                                )
                                .collect();
    let ranges: Vec<((i32, i32), (i32, i32))> = lines2.into_iter()
        .map(|v: Vec<String>| {
            (
                (v[0].parse::<i32>().unwrap(),
                v[1].parse::<i32>().unwrap()),
                (v[2].parse::<i32>().unwrap(),
                v[3].parse::<i32>().unwrap())
            )
        })
    .collect();
    
    

    return Answer{challenge1:Some(challenge1(&ranges)), challenge2: Some(challenge2(&ranges))};
}

fn within_check(range1: (i32, i32), range2: (i32, i32)) -> i32 {
    if range1.0 >= range2.0 && range1.1 <= range2.1{
        return 1;
    }
    else if range2.0 >= range1.0 && range2.1 <= range1.1{
        return 1;
    }
    return 0;
}


// In how many assignment pairs does one range fully contain the other?
fn challenge1(ranges: &Vec<((i32, i32),(i32, i32))>) -> i64{
    let within: Vec<i32> = ranges.into_iter()
        .map(|r| within_check(r.0, r.1)).collect();
    
    // why cant I check ranges here??!
    // println!("{}, {}, {}, {}", ranges[0].0.0, ranges[0].0.0, ranges[0].1.0, ranges[0].1.1);
    let mut sum: i32 = 0;
    within.into_iter().for_each(|x| sum += x);

    return sum as i64;
}


fn check_overlap(range1: (i32, i32), range2: (i32, i32)) -> i32 {
    // Range2 has a change to be within range1
    if range1.0 <= range2.0 {
        return max(min(range1.1 - range2.0, range2.1 - range2.0) + 1, 0);
    }
    return max( min(range2.1 - range1.0, range1.1 - range1.0) + 1, 0);
}


fn challenge2(ranges: &Vec<((i32, i32),(i32, i32))>) -> i64{
    let overlap: Vec<i32> = ranges.into_iter()
        .map(|r| check_overlap(r.0, r.1)).collect();
    
    let mut sum: i32 = 0;
    overlap.into_iter().for_each(|x| {
        if x > 0{
            sum += 1;
        }
    });

    return sum as i64;
}

#[cfg(test)]
mod day4_tests {
    use super::check_overlap;

    #[test]
    fn first_range_smaller(){
        let o = check_overlap((2,3), (4,5));
        assert_eq!(o, 0);
    }
    #[test]
    fn first_range_larger(){
        let o = check_overlap((4,5), (2,3));
        assert_eq!(o, 0);
    }
    #[test]
    fn border_first_range1(){
        let o = check_overlap((4,5), (1,4));
        assert_eq!(o, 1);
    }
    #[test]
    fn border_first_range2(){
        let o = check_overlap((4,5), (5,100));
        assert_eq!(o, 1);
    }
    #[test]
    fn first_range_completely_within_second(){
        let o = check_overlap((4,6), (1, 100));
        assert_eq!(o, 3);
    }

    #[test]
    fn second_range_completely_within_first(){
        let o = check_overlap((1,100), (10, 13));
        assert_eq!(o, 4);
    }

    #[test]
    fn first_range_overlapped_in_the_end(){
        let o = check_overlap((6, 9), (8, 13));
        assert_eq!(o, 2);
    }
    #[test]
    fn first_range_overlapped_in_the_beginning(){
        let o = check_overlap((6, 199), (3, 9));
        assert_eq!(o, 4);
    }

    #[test]
    fn one(){
        let o = check_overlap((5, 7), (7, 9));
        assert_eq!(o, 1);
    }
}