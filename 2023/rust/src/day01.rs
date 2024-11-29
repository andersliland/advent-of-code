// use std::fs::File;
// use std::io::{self, BufRead};
// use std::path::Path;
use std::io;

pub fn part1() -> Result<(), io::Error> {
    // let path = "01_input.txt";

    // let file = File::open(&path)?;
    // let reader = io::BufReader::new(file);

    let total_sum = 0;

    // for line in reader.lines() {
    //     let lint = line?;
    //     if let Some(calibration_value) = extract_calibration_value(&line) {
    //         total_sum += calibration_value;
    //     }
    // }

    println!("Total sum of calibration values: {}", total_sum);
    Ok(())
}

// fn extract_calibration_value(line: &str) -> Option<u32> {
//     let first_digit = line.chars().find(|c| c.is_digit(10))?;
//     let last_digit = line.chars().rev().find(|c| c.is_digit(10))?;

//     let first_digit = first_digit.to_digit(10)?;
//     let last_digit = last_digit.to_digit(10)?;

//     // Combine the first and last digits to form the two-digit number
//     Some(first_digit * 10 + last_digit)
// }
