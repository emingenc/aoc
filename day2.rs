use std::fs::File;
use std::io::{self, BufRead, BufReader};

fn check_sequence(numbers: &[i32]) -> bool {
    if numbers.len() < 2 {
        return false;
    }
    
    let first_diff = numbers[1] - numbers[0];
    let increasing = first_diff > 0;
    
    for i in 1..numbers.len() {
        let diff = numbers[i] - numbers[i-1];
        if (increasing && diff <= 0) || (!increasing && diff >= 0) {
            return false;
        }
        if diff.abs() > 3 || diff.abs() < 1 {
            return false;
        }
    }
    true
}

fn safe(numbers: &Vec<i32>) -> (bool, bool) {
    // First check if sequence is already safe
    let originally_safe = check_sequence(numbers);
    if originally_safe {
        return (true, false);
    }
    
    // Try removing each number to see if it becomes safe
    for i in 0..numbers.len() {
        let mut modified = numbers.clone();
        modified.remove(i);
        if check_sequence(&modified) {
            return (false, true);
        }
    }
    (false, false)
}

fn main() -> io::Result<()> {
    let file = File::open("2.txt")?;
    let reader = BufReader::new(file);
    let mut original_safe = 0;
    let mut fixable_safe = 0;

    for line in reader.lines() {
        let line = line?;
        let numbers: Vec<i32> = line
            .split_whitespace()
            .filter_map(|s| s.parse().ok())
            .collect();
        
        match safe(&numbers) {
            (true, _) => {
                original_safe += 1;
                println!("Originally safe sequence: {:?}", numbers);
            },
            (false, true) => {
                fixable_safe += 1;
                println!("Fixable sequence: {:?}", numbers);
            },
            (false, false) => {
                println!("Unsafe sequence: {:?}", numbers);
            }
        }
    }

    println!("Originally safe sequences: {}", original_safe);
    println!("Fixable safe sequences: {}", fixable_safe);
    println!("Total safe sequences: {}", original_safe + fixable_safe);
    Ok(())
}