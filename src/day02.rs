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
        
        let n_digits_lb = start.checked_ilog10().unwrap_or(0) + 1;
        let n_digits_end = end.checked_ilog10().unwrap_or(0) + 1;

        // check if both bounds have same number of digits and that number is odd then skip this range
        if n_digits_lb == n_digits_end && n_digits_lb%2 != 0 {
            println!("Range {} has same odd number of digits: {} and therefore cannot contain a double number", range, n_digits_lb);
            continue;
        }
        
    }
    
    Ok(())
}
