use std::io;

use std::fs::File;
use std::io::{BufRead};
use std::path::Path;

// A = Rock
// B = Paper
// C = Scissors

// points for the shape I select
// 1 for X (Rock)
// 2 for Y (Paper)
// 3 for Z (Scissors)

// X > C
// Y > A
// Z > B

// 
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


fn main() {
    let mut points1: u32 = 0;
    let mut points2: u32 = 0;

    if let Ok(lines) = read_lines("input.txt") {
        for line in lines {
            if let Ok(elf_me) = line{
                // println!("{}", elf_me);
                let split = elf_me.split(" ").collect::<Vec<&str>>();
                points1 += compete_part1(split[0], split[1]);
                points2 += compete_part2(split[0], split[1]);
            }
        }
    }
    println!("Part1 points {points1}");
    println!("Part2 points {points2}");
}


// The output is wrapped in a  Result to allow matching on errors
// Returns an Iterator to the Reader of the lines of the file
fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where P: AsRef<Path>, {
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}