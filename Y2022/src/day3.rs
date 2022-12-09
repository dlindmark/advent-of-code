use utils::{Answer, read_lines};

pub fn solve(filename: String) -> Answer<i64>{
    return Answer{challenge1: Some(challenge1(&filename)), challenge2: Some(challenge2(&filename))};
}

fn get_priority(item: char) -> i64 {
    let priority: i64;
    if item.is_lowercase(){
        priority = (item as u8 - 96) as i64;
    }
    else if item.is_uppercase() {
        priority = (item as u8 - 38) as i64;
    }
    else{
        priority = 0;
    }
    return priority;
}

fn challenge1(filename: &String) -> i64{
    let mut a1: i64 = 0;

    if let Ok(lines) = read_lines(filename) {
        // Consumes the iterator, returns an (Optional) String
        for line in lines {
            if let Ok(characters) = line {
                // println!("{}", characters.chars().nth(0).unwrap());
                // let pack1 = &characters[0..characters.len() / 2];
                let pack2: &str = &characters[characters.len() / 2..];
                for c in characters.chars() {
                    if pack2.contains(c) {
                        a1 += get_priority(c);
                        break;
                    }
                }
            }
        }
    }
    return a1
}


fn challenge2(filename: &String) -> i64{
    let mut a1: i64 = 0;

    if let Ok(lines) = read_lines(filename) {
        let mut peekable_lines = lines.peekable();
        while peekable_lines.peek().is_some(){
            let line1 = peekable_lines.next().unwrap().unwrap();
            let line2 = peekable_lines.next().unwrap().unwrap();
            let line3 = peekable_lines.next().unwrap().unwrap();
            for c in line1.chars() {
                if line2.contains(c) {
                    if line3.contains(c){
                        a1 += get_priority(c);
                        break;
                    }
                }
            }
        }
    }
    return a1
}