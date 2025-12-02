use std::env;
mod day01;
mod day02;

fn main() {
    let args: Vec<String> = env::args().collect();
    let day: &String  = &args[1];
    let day: u32 = day.parse().expect("Please provide a valid day number");

    match day {
        1 => { let _ = day01::day01(); }
        2 => { let _ = day02::day02(); }
        // Add more days as you implement them:
        // 2 => { let _ = day02::day02(); }
        _ => {
            println!("Day {day} is not implemented.");
        }
    }
}