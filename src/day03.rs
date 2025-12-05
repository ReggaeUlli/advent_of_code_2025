use std::fs::File;
use std::io::{BufRead, BufReader};
use std::path::Path;
use std::error::Error;

pub fn day03(challange:i32) -> Result<(), Box<dyn Error>>{
    println!("Day 03, Challange {}", challange);
    // Specify the path to the text file
    let file_path = Path::new("input/03.txt");

    // Open the file
    let file = File::open(file_path)?;
    let reader = BufReader::new(file);

    println!("Reading file line-by-line:\n");
    // Read each line from the file
    let mut sum:i64 = 0; 
    for line in reader.lines() {
        let line = line?;
        let chars: Vec<char> = line.chars().collect();
        if challange == 1{
            let mut highest_digit = 0;
            let mut highest_index = 0;
            for (i, c) in chars.iter().enumerate() {
                if i >= chars.len()-1{
                    break;
                }
                if let Some(digit) = c.to_digit(10) {
                    if digit > highest_digit {
                        highest_digit = digit;
                        highest_index = i;
                    }
                }
            }
            let mut second_highest = 0;
            for (j, c) in chars.iter().enumerate() {
                if j > highest_index {
                    if let Some(digit) = c.to_digit(10) {
                        if digit > second_highest {
                            second_highest = digit;
                        }
                    }
                }
            }
            println!("{}: {}{}", line, highest_digit, second_highest);
            sum += (highest_digit*10 + second_highest)as i64;
        }
        else if challange == 2{
            let mut line_sum:i64 = 0;
            let max = 12;
            let mut last_j = 0; 
            for i in (0..max).rev(){
              let mut i_highest_digit:i64 = 0;
              // iterate until len-i to make sure enough chars remain for complete number
              if i != max-1 {
                last_j += 1;
              }
              for j in last_j..chars.len()-i{
                if let Some(d) = chars[j].to_digit(10) {
                        let d_i64 = d as i64;
                        if d_i64 > i_highest_digit {
                            i_highest_digit = chars[j].to_digit(10).unwrap() as i64;
                            last_j = j;
                        }
                     
                }
                }
                line_sum += i_highest_digit * 10i64.pow(i as u32);
            }
            println!("{}: {}", line, line_sum);
            sum += line_sum;
        }
    }
    println!("Sum: {}", sum);
    Ok(())
}