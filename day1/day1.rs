use std::fs::File;
use std::io::{self, BufRead, BufReader};
use std::collections::HashMap; 


fn main() -> io::Result<()> {
    let file = File::open("1.txt")?;
    let reader = BufReader::new(file);
    
    let mut first_numbers = Vec::new();
    let mut second_numbers = Vec::new();
    let mut count_map: HashMap<i32, i32> = HashMap::new();
    
    for line in reader.lines() {
        let line = line?;
        let numbers: Vec<&str> = line.split_whitespace().collect();
        
        if numbers.len() == 2 {
            if let (Ok(first), Ok(second)) = (
                numbers[0].parse::<i32>(),
                numbers[1].parse::<i32>()
            ) {
                first_numbers.push(first);
                second_numbers.push(second);
                *count_map.entry(second).or_insert(0) += 1;
            }
        }
    }
    
    first_numbers.sort();
    second_numbers.sort();

    let mut total = 0;
    
    for (first, second) in first_numbers.iter().zip(second_numbers.iter()) {
        let diff = (first - second).abs(); 
        total += diff; 
        println!("First: {}, Second: {}, Diff: {}", first, second, diff);
    }
    
    println!("Total sum of differences: {}", total);

    let mut total2 = 0;

    for number in &first_numbers {
        if let Some(count) = count_map.get(number) {
            total2 += number * count;
        }
    }

    println!("Total sum of numbers: {}", total2);
    
    Ok(())
}
