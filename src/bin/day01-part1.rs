
fn _main() {
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

fn _print_type_of<T>(_: T) {
    println!("{}", std::any::type_name::<T>())
}


///
/// https://www.youtube.com/watch?v=JOgQMjpGum0&ab_channel=chrisbiscardi
/// - by chris biscardi
///
fn main() {
    let str = include_str!("./input-day1.txt");

    let lines = str.lines();

    let t = lines.map(|line| {
        // dbg!(line);
        let mut digits = line.chars().filter_map(|ch| {
            ch.to_digit(10)
        });

        // dbg!(line);

        let first = digits.next().expect("not a number");
        if let Some(last) = digits.last() {
            first * 10 + last
        } else {
            first * 10 + first
        }
    }).sum::<u32>();

    //println!("{:?}", std::any::type_name(t));
    //println!("{:?}", print_type_of(t));
    println!("{}", t);
}