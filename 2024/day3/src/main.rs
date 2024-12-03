use std::fs::File;
use std::io::{self, BufRead, BufReader};

fn parse_number(chars: &[char], start: &mut usize) -> Option<i32> {
    let mut num = String::new();
    while *start < chars.len() && chars[*start].is_digit(10) {
        num.push(chars[*start]);
        *start += 1;
    }
    num.parse().ok()
}

fn extract_multiplication(chars: &[char], pos: &mut usize) -> Option<i32> {
    // Check for "mul("
    if *pos + 4 > chars.len() {
        return None;
    }
    if chars[*pos..*pos + 4].iter().collect::<String>() != "mul(" {
        return None;
    }
    *pos += 4; // Skip "mul("
    
    // Parse first number
    let num1 = parse_number(chars, pos)?;
    
    // Check for comma
    if *pos >= chars.len() || chars[*pos] != ',' {
        return None;
    }
    *pos += 1;
    
    // Parse second number
    let num2 = parse_number(chars, pos)?;
    
    // Check for closing parenthesis
    if *pos >= chars.len() || chars[*pos] != ')' {
        return None;
    }
    *pos += 1;
    
    Some(num1 * num2)
}

fn extract_multiplications(line: &str, total: &mut i32, total2: &mut i32, enabled: &mut bool) {
    let chars: Vec<char> = line.chars().collect();
    let mut pos = 0;
    
    while pos < chars.len() {
        // Check for "don't()" (7 characters)
        if pos + 7 <= chars.len() && chars[pos..pos + 7].iter().collect::<String>() == "don't()" {
            *enabled = false;
            println!("Found don't() at position {}, setting enabled to false", pos);
            pos += 7;
        }
        // Check for "do()" (4 characters)
        else if pos + 4 <= chars.len() && chars[pos..pos + 4].iter().collect::<String>() == "do()" {
            *enabled = true;
            println!("Found do() at position {}, setting enabled to true", pos);
            pos += 4;
        }
        // Check for "mul(x,y)"
        else if let Some(result) = extract_multiplication(&chars, &mut pos) {
            *total += result;
            if *enabled {
                *total2 += result;
                println!("Adding {} to total2 (enabled)", result);
            } else {
                println!("Skipping {} as enabled is false", result);
            }
            println!("Current totals -> total: {}, total2: {}", total, total2);
        } else {
            pos += 1;
        }
    }
}

fn main() -> io::Result<()> {
    let file = File::open("3.txt")?;
    let reader = BufReader::new(file);
    let mut total = 0;
    let mut total2 = 0;
    let mut enabled = true;

    for line in reader.lines() {
        let line = line?;
        extract_multiplications(&line, &mut total, &mut total2, &mut enabled);
    }

    println!("Total sum of all multiplications: {}", total);
    println!("Total sum of enabled multiplications: {}", total2);
    Ok(())
}