use utils;

pub fn day1(filename: String) -> utils::Answer{
    let mut max_calories: [i32; 3] = [0, 0, 0];
    let mut current_calories: i32 = 0;

    // File hosts must exist in current path before this procedure output
    if let Ok(lines) = utils::read_lines(filename) {
        // Consumes the iterator, returns an (Optional) String
        for line in lines {
            if let Ok(number) = line {
                // Check if new line
                if number.trim().is_empty() {
                    max_calories.sort();
                    for i in 0..max_calories.len() {
                        if current_calories > max_calories[i]{
                            max_calories[i] = current_calories;
                            break;
                        }
                    }
                    current_calories = 0;
                }
                else{
                    let snack = number.trim().parse::<i32>().unwrap();
                    current_calories += snack;
                }
            }
        }
    }
    max_calories.sort();
    return utils::Answer { challenge1: max_calories[2] as i64, challenge2: (max_calories[0]+max_calories[1] + max_calories[2]) as i64 }
}