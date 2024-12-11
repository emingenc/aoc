use std::collections::HashMap;
use std::fs;
use num_bigint::BigUint;
use num_traits::{Zero, One};

fn main() {
    let input = fs::read_to_string("input.txt").expect("Failed to read input.txt");
    let initial_stones: Vec<BigUint> = input
        .split_whitespace()
        .map(|s| s.trim().parse::<BigUint>().unwrap())
        .collect();

    // Part 1: After 25 blinks
    let total_stones_part1 = simulate_blinks(&initial_stones, 25);
    println!("Part 1: Total stones after 25 blinks: {}", total_stones_part1);

    // Part 2: After 75 blinks
    let total_stones_part2 = simulate_blinks(&initial_stones, 75);
    println!("Part 2: Total stones after 75 blinks: {}", total_stones_part2);
}

fn simulate_blinks(initial_stones: &Vec<BigUint>, blinks: usize) -> usize {
    let mut counts: HashMap<BigUint, usize> = HashMap::new();
    for stone in initial_stones {
        *counts.entry(stone.clone()).or_insert(0) += 1;
    }

    for _ in 0..blinks {
        let mut new_counts: HashMap<BigUint, usize> = HashMap::new();

        for (stone, &count) in &counts {
            if stone.is_zero() {
                // Rule 1: 0 becomes 1
                let one = BigUint::one();
                *new_counts.entry(one).or_insert(0) += count;
            } else if stone.to_string().len() % 2 == 0 {
                // Rule 2: Split the number
                let num_str = stone.to_string();
                let mid = num_str.len() / 2;

                let left_str = &num_str[..mid];
                let right_str = &num_str[mid..];

                let left = left_str.trim_start_matches('0').parse::<BigUint>().unwrap_or_else(|_| BigUint::zero());
                let right = right_str.trim_start_matches('0').parse::<BigUint>().unwrap_or_else(|_| BigUint::zero());

                *new_counts.entry(left).or_insert(0) += count;
                *new_counts.entry(right).or_insert(0) += count;
            } else {
                // Rule 3: Multiply by 2024
                let new_stone = stone * BigUint::from(2024u32);
                *new_counts.entry(new_stone).or_insert(0) += count;
            }
        }
        counts = new_counts;
    }

    counts.values().sum()
}