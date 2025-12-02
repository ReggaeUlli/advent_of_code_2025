use std::fs::File;
use std::io::{BufRead, BufReader};
use std::path::Path;
use std::error::Error;

pub fn day02() -> Result<(), Box<dyn Error>>{

    // Specify the path to the text file
    let file_path = Path::new("input/02.txt");

    // Open the file
    let file = File::open(file_path)?;
    let reader = BufReader::new(file);

    println!("Reading file line-by-line:\n");
    // Read each line from the file 
    let line = reader.lines().next().unwrap().unwrap();
    let ranges = line.split(",");

    let mut result = 0;
    for range in ranges {
        let bounds: Vec<&str> = range.split("-").collect();
        let start: i64 = bounds[0].parse().unwrap();
        let end: i64 = bounds[1].parse().unwrap();
        println!("Range: {}, Start: {}, End: {}", range, start, end);
        
        let n_digits_start = start.checked_ilog10().unwrap_or(0) + 1;
        let n_digits_end = end.checked_ilog10().unwrap_or(0) + 1;

        // check if both bounds have same number of digits and that number is odd then skip this range
        if n_digits_start == n_digits_end && n_digits_start%2 != 0 {
            println!("Range {} has same odd number of digits: {} and therefore cannot contain a double number", range, n_digits_start);
            continue;
        }
        for i in n_digits_start..=n_digits_end{
            let digit_lower_bound: i64;
            if i == n_digits_start{
                digit_lower_bound = start;
            }else{
                digit_lower_bound = 10i64.pow((i-1) as u32);
            }
            let digit_upper_bound: i64;
            if i == n_digits_end{
                digit_upper_bound = end;
            }else{
                digit_upper_bound = 10i64.pow(i as u32) - 1;
            }
            for number in digit_lower_bound..=digit_upper_bound{
                let number_str = number.to_string();
                let chars: Vec<char> = number_str.chars().collect();
                let halfpoint = (chars.len() as f64 / 2.0).ceil() as usize;
                if &chars[..halfpoint] == &chars[halfpoint..]{
                    result += number;
                }
            }
        }
    }
    println!("Result: {}", result);
    Ok(())
}