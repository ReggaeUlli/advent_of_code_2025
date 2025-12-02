use std::fs::File;
use std::io::{BufRead, BufReader};
use std::path::Path;
use std::error::Error;

pub fn day01() -> Result<(), Box<dyn Error>>{
    // Specify the path to the text file
    let file_path = Path::new("input/01.txt");

    // Open the file
    let file = File::open(file_path)?;
    let reader = BufReader::new(file);

    println!("Reading file line-by-line:\n");
    let mut x = 50;
    let mut result = 0;
    // Read each line from the file
    for line_result in reader.lines() {
        let line = line_result?;
        let dir = &line[..1];
        let amount = &line[1..].parse::<i32>().unwrap();
        let old_x = x;
        
        //move
        if dir == "L"{
            x = x-amount;
        }
        else{
            x = x+amount;
        }

        println!("line: {}, new_x:{}", line, x);
        
        let mut n_xxx00_pass = 0;
        let new_x_100th = x as f64 / 100.0;
        let old_x_100th = old_x as f64 / 100.0;
        
        // if we pass absolute zero add one as this is not catched by later check
        // -250 -> 250 would result in 4, but actually 5 zeros are crossed
        // (200, 100, 0, 100, 200)
        //also -50 -> 50 would result in 0 but we crossed the absolute zero
        //to not loose the +- info we need them as floats at start
        if (old_x_100th<0.0 && new_x_100th > 0.0)
            || (old_x_100th>0.0 && new_x_100th < 0.0){
                n_xxx00_pass +=1;
        } 

        // now can convert to int
        let new_x_100th_int = new_x_100th as i32;
        let old_x_100th_int = old_x_100th as i32;
        //get number of passed XXX00 crossings
        n_xxx00_pass += (new_x_100th_int - old_x_100th_int).abs();
        if n_xxx00_pass != 0 {
            // If we start at XXX00 then reduce n_xxx00_pass by one
            // as this 0 was already count last time
            if old_x%100 == 0{
                if old_x<=0 && old_x<x{
                    n_xxx00_pass -=1;
                } else if old_x>=0 && old_x>x{
                    n_xxx00_pass -=1;
                } 
            }
            if n_xxx00_pass !=0{
                result = result + n_xxx00_pass.abs();
                println!("BINGO:{}", result);
            }   
        }
        // if we go in positive numbers downwards, add +1 if hitting 100 as no 100th crossing is detected for example in 170 -> 100
        // but if we go from 150 to 200 this is already detected by the n_xxx00_pass logic
        // for negative numbers the same logic applies but vice versa -170 -> -100 is not a crossing but -150 -> -200 is detected by n_xxx00_pass
        if x%100==0 && (x>=0 && old_x>x || x<=0 && old_x<x){
            result = result +1;
            println!("BINGO:{}", result);
        }
    }
    println!("{}", result);
    Ok(())
}
