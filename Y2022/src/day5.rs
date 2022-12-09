use utils::{Answer, collect_lines_in_file};
use std::{collections::VecDeque};

pub fn solve(filename: String) -> Answer<String>{
    let mut stacks: [VecDeque<char>; 9] = Default::default();
    let mut instructions: Vec<Instruction> = Vec::new();
    let mut stacks2: [VecDeque<char>; 9] = Default::default();
    let mut instructions2: Vec<Instruction> = Vec::new();
    parse_into_queues_and_instructions(filename.clone(), &mut stacks, &mut instructions);
    parse_into_queues_and_instructions(filename, &mut stacks2, &mut instructions2);
    follow_instructions_on_stack_challenge1(instructions, &mut stacks);
    follow_instructions_on_stack_challenge2(instructions2, &mut stacks2);

    let mut a1: String = String::from("");
    let mut a2: String = String::from("");
    for stack in stacks {
        a1.push(stack[0]);
    }
    for stack in stacks2 {
        a2.push(stack[0]);
    }
    return Answer{challenge1:Some(a1), challenge2:Some(a2)};
}

fn follow_instructions_on_stack_challenge1(instructions: Vec<Instruction>, stacks: &mut [VecDeque<char>]){
    for instruction in instructions {
        for _ in 0..instruction.nb {
            let ch: char = stacks[instruction.from_stack-1].pop_front().unwrap();
            stacks[instruction.to_stack-1].push_front(ch);
        }
    }
}

fn follow_instructions_on_stack_challenge2(instructions: Vec<Instruction>, stacks: &mut [VecDeque<char>]){
    for instruction in instructions {
        let  mut tmp: VecDeque<char> = Default::default();
        for _ in 0..instruction.nb {
            let ch: char = stacks[instruction.from_stack-1].pop_front().unwrap();
            tmp.push_front(ch);
        }
        for ch in tmp{
            stacks[instruction.to_stack-1].push_front(ch);
        }
    }
}

#[derive(Debug)]
struct Instruction {
    nb: i64,
    from_stack: usize,
    to_stack: usize
}

fn count_leading_whitespace(s: &str) -> usize {
    return s.chars().take_while(|ch: &char| ch.is_whitespace() && *ch != '\n').count();
}

fn parse_into_queues_and_instructions(filename: String, stacks: &mut [VecDeque<char>], instructions: &mut Vec<Instruction>) {
    let lines: Vec<String> = collect_lines_in_file(filename);

    let line_nb = parse_stacks(&lines, stacks);

    for line in lines[line_nb..].into_iter(){
        let numbers: Vec<i64> = line.split(' ').filter_map(|s| s.parse::<i64>().ok()).collect();
        instructions.push(Instruction { nb: numbers[0], from_stack: numbers[1] as usize, to_stack: numbers[2] as usize})
    }
}

fn parse_stacks(lines: &Vec<String>, stacks: &mut [VecDeque<char>]) -> usize {
    let mut instruction_starts_at: usize = 0;
    for (i, line) in lines.iter().enumerate() {
        // if true the next line starts the instructions
        if line.contains(" 1"){
            instruction_starts_at = i + 2;
            break;
        }
        let mut ws = count_leading_whitespace(&line);
        let mut nb = ws / 4;
        let mut s = line.trim();
        let mut stop = false;
        while !stop {
            let ch = s.chars().nth(1).unwrap();
            stacks[nb].push_back(ch);
            s = &s[3..];
            if s.len() <= 0 {
                stop = true;
            }
            ws = count_leading_whitespace(s);
            nb += ws / 4 + 1;
            s = s.trim();
        }
    }

    return instruction_starts_at
}


#[cfg(test)]
mod day5_tests {
    use super::parse_into_queues_and_instructions;
    use super::Instruction;
    use std::collections::VecDeque;

    #[test]
    fn test_stack_row_1(){
        let mut stacks: [VecDeque<char>; 9] = Default::default();
        let mut instructions: Vec<Instruction> = Vec::new();
        let facit: Vec<char> = Vec::from(['B', 'Q', 'H', 'D', 'S', 'L', 'R', 'T']);
        parse_into_queues_and_instructions("day5.txt".to_string(), &mut stacks, &mut instructions);
        for c in stacks[4].iter().zip(facit.iter()){
            let (c1, c2) = c;
            println!("{}, {}", c1, c2);
            assert_eq!(c1, c2);
        }
    }
    
    #[test]
    fn test_instructions(){
        let mut stacks: [VecDeque<char>; 9] = Default::default();
        let mut instructions: Vec<Instruction> = Vec::new();
        let facit: Vec<Instruction> = Vec::from([
            Instruction{nb: 5, from_stack: 4, to_stack: 5},
            Instruction{nb: 2, from_stack: 5, to_stack: 8},
            Instruction{nb: 2, from_stack: 9, to_stack: 1},
            ]);
        parse_into_queues_and_instructions("day5.txt".to_string(), &mut stacks, &mut instructions);
        for c in instructions.iter().zip(facit.iter()){
            let (c1, c2) = c;
            assert_eq!(c1.nb, c2.nb);
            assert_eq!(c1.from_stack, c2.from_stack);
            assert_eq!(c1.to_stack, c2.to_stack);
        }
    }

}