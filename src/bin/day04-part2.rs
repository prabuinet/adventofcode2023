use core::num;
use std::collections::HashSet;


fn get_card_number(str: &str) -> i32 {
    let s: Vec<&str> = str.split(' ').map(|x| x.trim()).filter(|x| x.len() > 0).collect();
    s[1].parse::<i32>().unwrap()
}

#[derive(Debug, Clone)]
struct Card {
    number: i32,
    count: i32,
    scratched: i32,
    score: i32,
}

fn _main1() {
    let str = include_str!("./input-day4.txt");

    let lines = str.lines();

    let mut cards: Vec<Card> = Vec::<Card>::new();

    for line in lines {
        let ls: Vec<&str> = line.split(':').collect();

        // println!("{}", get_game_number(&s[0]));
        let card_number = get_card_number(&ls[0]);

        let card_split: Vec<&str> = ls[1].split('|').collect();
        let winning_numbers: HashSet<i32> = card_split[0].split(' ').map(|x| x.trim()).filter(|x| x.len() > 0).map(|x| x.parse::<i32>().expect(format!("{x}").as_str())).collect();
        let available_numbers: Vec<i32> = card_split[1].split(' ').map(|x| x.trim()).filter(|x| x.len() > 0).map(|x| x.parse::<i32>().unwrap()).collect();

        let mut score = 0;

        for a in available_numbers {
            if winning_numbers.contains(&a) {
                score += 1;
            }
        }

        cards.push(Card {
            number: card_number,
            count: 1,
            scratched: 0,
            score: score
        })
    }

    let mut iterations: u128 = 0;

    loop {
        iterations += 1;

        let unscratched: Vec<(i32, i32)> = cards.iter().filter(|c| c.scratched < c.count).map(|c| (c.number, c.score)).collect();

        println!("{}", unscratched.clone().len());

        if unscratched.clone().len() == 0 {
            break;
        }

        for un in unscratched.clone() {
            // uc.scratched += 1;
            if un.1 > 0 {
                let winning_cards: Vec<&mut Card> = cards.iter_mut().filter(|c| c.number > un.0 && c.number <= (un.0 + un.1)).collect();
                for wc in winning_cards {
                    wc.count += 1;
                }
            }
        }

        let unscratched_cards: Vec<&mut Card> = cards.iter_mut().filter(|c| unscratched.clone().iter().any(|x| x.0 == c.number)).collect();
        for uc in unscratched_cards {
            uc.scratched += 1;
        }
    }

    println!("{}", cards.iter().map(|x| x.count as i32).sum::<i32>());
    println!("{}", iterations);
    //let line: Option<&str> = lines.nth(0);
}


// 10212704
// 10212704

fn main() {
    let str = include_str!("./input-day4.txt");

    let lines = str.lines();

    let mut cards: Vec<Card> = Vec::<Card>::new();

    for line in lines {
        let ls: Vec<&str> = line.split(':').collect();

        // println!("{}", get_game_number(&s[0]));
        let card_number = get_card_number(&ls[0]);

        let card_split: Vec<&str> = ls[1].split('|').collect();
        let winning_numbers: HashSet<i32> = card_split[0].split(' ').map(|x| x.trim()).filter(|x| x.len() > 0).map(|x| x.parse::<i32>().expect(format!("{x}").as_str())).collect();
        let available_numbers: Vec<i32> = card_split[1].split(' ').map(|x| x.trim()).filter(|x| x.len() > 0).map(|x| x.parse::<i32>().unwrap()).collect();

        let mut score = 0;

        for a in available_numbers {
            if winning_numbers.contains(&a) {
                score += 1;
            }
        }

        cards.push(Card {
            number: card_number,
            count: 1,
            scratched: 0,
            score: score
        })
    }

    let count = cards.len();

    let mut i = 0;

    while i < count {
        let card = cards.get(i).unwrap();
        let score = card.score;
        let ccount = card.count;
        println!("{}", card.number);

        for j in (card.number + 1)..(card.number + 1 + score) {
            if let Some(cd) = cards.get_mut((j - 1) as usize) {
                cd.count += ccount;
            }
        }

        i += 1;
    }

    println!("{}", cards.iter().map(|x| x.count as i32).sum::<i32>());
}
