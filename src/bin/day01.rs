
fn _main1() {
    let str = include_str!("./input-day1.txt");

    let mut sum = 0;

    for line in str.lines() {
        let mut first = 0;
        let mut last = 0;

        for ch in line.chars() {
            if ch >= '0' && ch <= '9' {
                first = ch.to_digit(10).unwrap() - '0'.to_digit(10).unwrap();
                break;
            }
        }

        for ch in line.chars().rev() {
            if ch >= '0' && ch <= '9' {
                last = ch.to_digit(10).unwrap() - '0'.to_digit(10).unwrap();
                break;
            }
        }


        sum += (first * 10) + last;
    }

    println!("{}", sum);

}


fn main() {
    let str = include_str!("./input-day1.txt");

    let nums = ["one", "two", "three", "four", "five", "six", "seven", "eight", "nine"];


    let mut sum = 0;

    for line in str.lines() {
        let mut first: u32 = 0;
        let mut last: u32 = 0;

        for i in 0..line.len() {
            if let Some(ch) = line.chars().nth(i) {
                if ch >= '0' && ch <= '9' {
                    first = ch.to_digit(10).unwrap() - '0'.to_digit(10).unwrap();
                    break;
                }
                else {
                    let mut found_num: bool = false;

                    for (j, num) in nums.iter().enumerate() {
                        if i + num.len() - 1 < line.len() {
                            let subs = &line[i..(i + num.len())];
                            if subs == *num {
                                first = (j as u32) + 1;
                                found_num = true;
                                break;
                            }
                        }
                    }

                    if found_num {
                        break;
                    }
                }
            }
        }

        for i in (0..line.len()).rev() {
            if let Some(ch) = line.chars().nth(i) {

                if ch >= '0' && ch <= '9' {
                    last = ch.to_digit(10).unwrap() - '0'.to_digit(10).unwrap();
                    break;
                }
                else {
                    let mut found_num: bool = false;

                    for (j, num) in nums.iter().enumerate() {
                        if i + num.len() - 1 < line.len() {
                            let subs = &line[i..(i + num.len())];
                            if subs == *num {
                                last = (j as u32) + 1;
                                found_num = true;
                                break;
                            }
                        }
                    }

                    if found_num {
                        break;
                    }
                }
            }
        }


        sum += (first * 10) + last;
        // println!("{} {} {}", line, first, last);
    }

    println!("{}", sum);

}
