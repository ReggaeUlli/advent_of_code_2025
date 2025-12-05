use std::env;
mod day01;
mod day02;
mod day03;

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() < 3 {
        println!("Usage: {} <day> <challange>", args[0]);
        return;
    }
    let day: &String  = &args[1];
    let challange = &args[2];
    let day: u32 = day.parse().expect("Please provide a valid day number");

    match day {
        1 => { let _ = day01::day01(challange.parse().unwrap()); }
        2 => { let _ = day02::day02(challange.parse().unwrap()); }
        3 => { let _ = day03::day03(challange.parse().unwrap()); }
        // Add more days as you implement them:
        // 2 => { let _ = day02::day02(); }
        _ => {
            println!("Day {day} is not implemented.");
        }
    }
}