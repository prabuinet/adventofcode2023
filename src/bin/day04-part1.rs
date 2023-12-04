use std::collections::HashSet;


fn get_card_number(str: &str) -> i32 {
    let s: Vec<&str> = str.split(' ').map(|x| x.trim()).filter(|x| x.len() > 0).collect();
    s[1].parse::<i32>().unwrap()
}

fn main() {
    let str = include_str!("./input-day4.txt");

    let lines = str.lines();
    let mut sum = 0;

    for line in lines {
        let ls: Vec<&str> = line.split(':').collect();

        // println!("{}", get_game_number(&s[0]));
        let _card_number = get_card_number(&ls[0]);

        let card_split: Vec<&str> = ls[1].split('|').collect();
        let winning_numbers: HashSet<i32> = card_split[0].split(' ').map(|x| x.trim()).filter(|x| x.len() > 0).map(|x| x.parse::<i32>().expect(format!("{x}").as_str())).collect();
        let available_numbers: Vec<i32> = card_split[1].split(' ').map(|x| x.trim()).filter(|x| x.len() > 0).map(|x| x.parse::<i32>().unwrap()).collect();

        let mut score = 0;

        for a in available_numbers {
            if winning_numbers.contains(&a) {
                if score == 0 {
                    score = 1;
                } else {
                    score *= 2;
                }
            }
        }

        sum += score;
    }


    println!("{sum}");
    //let line: Option<&str> = lines.nth(0);


}