use std::cmp;


fn _get_game_number(str: &str) -> i32 {
    let s: Vec<&str> = str.split(' ').collect();
    s[1].parse::<i32>().unwrap()
}

fn get_cubes_and_color(str: &str) -> (i32, &str) {
    let s: Vec<&str> = str.split(' ').collect();
    (s[0].parse::<i32>().unwrap(), s[1])
}


fn main() {
    let str = include_str!("./input-day2.txt");

    let lines = str.lines();
    let mut sum = 0;

    for line in lines {
        let ls: Vec<&str> = line.split(':').collect();

        // println!("{}", get_game_number(&s[0]));
        // let game_number = get_game_number(&ls[0]);

        let reveals: Vec<&str> = ls[1].split(';').collect();

        let mut rmax = 0;
        let mut gmax = 0;
        let mut bmax = 0;

        for r in reveals {
            let cubes: Vec<&str> = r.split(',').collect();
            let cubes: Vec<(i32, &str)> = cubes.iter().map(|str| get_cubes_and_color(str.trim())).collect();

            for x in cubes {
                let (x, str) = x;
                match str {
                    "red" => rmax = cmp::max(rmax, x),
                    "green" => gmax = cmp::max(gmax, x),
                    "blue" => bmax = cmp::max(bmax, x),
                    _ => todo!(),
                };
            }
        }


        sum += rmax * gmax * bmax;
    }

    println!("{sum}");
    //let line: Option<&str> = lines.nth(0);


}