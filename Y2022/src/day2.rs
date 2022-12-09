use utils::{read_lines, Answer};

fn compete_part1(elf: &str, me: &str) -> u32{
    let mut point: u32 = 0;
    if me == "X"{
        point += 1u32;
        if elf == "A" {
            point += 3u32;
        }
        else if elf == "C" {
            point += 6u32;
        }
    }
    else if me == "Y"{
        point += 2u32;
        if elf == "B" {
            point += 3u32;
        }
        else if elf == "A" {
            point += 6u32;
        }
    }
    else if me == "Z"{
        point += 3u32;
        if elf == "C" {
            point += 3u32;
        }
        else if elf == "B" {
            point += 6u32;
        }
    }
    return point;
}

fn compete_part2(elf: &str, me: &str) -> u32{
    let mut point: u32 = 0;
    if me == "X"{
        if elf == "A" {
            point += 3u32;
        }
        else if elf == "B" {
            point += 1u32;
        }
        else if elf == "C"{
            point += 2u32;
        }
    }
    else if me == "Y"{
        point += 3u32;
        if elf == "A" {
            point += 1u32;
        }
        else if elf == "B" {
            point += 2u32;
        }
        else if elf == "C"{
            point += 3u32;
        }
    }
    else if me == "Z"{
        point += 6u32;
        if elf == "A" {
            point += 2u32;
        }
        else if elf == "B" {
            point += 3u32;
        }
        else if elf == "C"{
            point += 1u32;
        }
    }
    return point;
}

pub fn solve(filename: String) -> Answer<i64> {
    let mut points1: u32 = 0;
    let mut points2: u32 = 0;

    if let Ok(lines) = read_lines(filename) {
        for line in lines {
            if let Ok(elf_me) = line{
                // println!("{}", elf_me);
                let split = elf_me.split(" ").collect::<Vec<&str>>();
                points1 += compete_part1(split[0], split[1]);
                points2 += compete_part2(split[0], split[1]);
            }
        }
    }
    return Answer {challenge1: Some(points1 as i64), challenge2: Some(points2 as i64)};
}