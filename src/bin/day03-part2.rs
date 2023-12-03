
fn get_nums(line: &str) -> Vec<(u32, u32)> {
    let mut result: Vec<(u32, u32)> = Vec::<(u32, u32)>::new();

    let mut i  = 0;
    let mut start: i32 = -1;

    while i < line.len() {
        if line.chars().nth(i).unwrap().is_digit(10) {
            if start == -1 {
                start = i as i32;
            }
        } else {
            if start != -1 {
                result.push((start as u32, (i - 1) as u32));
                start = -1;
            }
        }
        i += 1;
    }

    if i == line.len() && start != -1 {
        result.push((start as u32, (i - 1) as u32));
    }

    result
}

fn main() {
    let str = include_str!("./input-day3.txt");

    let lines: Vec<&str> = str.lines().collect();
    let mut line_number = 0;
    let mut sum: u128 = 0;

    let mut num_positions: Vec::<Vec<(u32, u32)>> = Vec::<Vec<(u32, u32)>>::new();

    while line_number < lines.len() {
        let positions = get_nums(lines.get(line_number).unwrap());
        num_positions.push(positions);
        line_number += 1;
    }

    line_number = 0;
    while line_number < lines.len() {
        let line = lines[line_number].to_string();

        let mut j = 0;

        // print!("{}:    ", lines[line_number]);

        while j < line.len() {
            if line.chars().nth(j).unwrap() == '*' {
                let gear_nums: Vec<u64> = get_gear(&lines, &num_positions, line_number, j);
                if gear_nums.len() == 2 {
                    sum += gear_nums[0] as u128 * gear_nums[1] as u128;
                }

                // print!("{:?}", gear_nums);
            }

            j += 1;
        }

        // println!("");
        line_number += 1;
    }


    println!("{}", sum);
}

fn get_gear(lines: &Vec<&str>, num_positions: &Vec<Vec<(u32, u32)>>, line_number: usize, col: usize) -> Vec<u64> {

    let mut gear_nums: Vec<u64> = Vec::new();

    // check above
    if line_number > 0 {
        get_gears_in_line(line_number - 1, col, num_positions, &lines, &mut gear_nums);
    }

    get_gears_in_line(line_number, col, num_positions, &lines, &mut gear_nums);

    if line_number < lines.len() - 1 {
        get_gears_in_line(line_number + 1, col, num_positions, &lines, &mut gear_nums);
    }

    gear_nums
}

fn get_gears_in_line(line_number: usize, col: usize, num_positions: &Vec<Vec<(u32, u32)>>, lines: &Vec<&str>, gear_nums: &mut Vec<u64>) {
    let from = col - 1;

    for nums in &num_positions[line_number] {
        if (nums.0 >= from as u32 && nums.0 <= (from + 2) as u32) ||
           (nums.1 >= from as u32 && nums.1 <= (from + 2) as u32)  {
            let line = lines[line_number].to_string();
            let num = (&line[nums.0 as usize .. (nums.1 + 1) as usize]).parse::<u64>();
            gear_nums.push(num.unwrap());
        }
    }
}

