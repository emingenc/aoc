use std::collections::{HashMap, HashSet};
use std::fs::File;
use std::io::{BufRead, BufReader};

fn gcd(a: i32, b: i32) -> i32 {
    if b == 0 {
        a.abs()
    } else {
        gcd(b, a % b)
    }
}

fn main() {
    // Open the input file
    let file = File::open("input.txt").expect("Cannot open input.txt");
    let reader = BufReader::new(file);

    // Read the input map
    let map: Vec<Vec<char>> = reader
        .lines()
        .map(|line| line.unwrap().chars().collect())
        .collect();

    let height = map.len() as i32;
    let width = if height > 0 { map[0].len() as i32 } else { 0 };

    // Collect antenna positions grouped by frequency
    let mut antennas: HashMap<char, Vec<(i32, i32)>> = HashMap::new();
    for (y, row) in map.iter().enumerate() {
        for (x, &ch) in row.iter().enumerate() {
            if ch != '.' {
                antennas
                    .entry(ch)
                    .or_insert_with(Vec::new)
                    .push((x as i32, y as i32));
            }
        }
    }

    // Part One
    let mut antinodes_part1 = HashSet::new();

    // For each frequency
    for positions in antennas.values() {
        // For each pair of antennas
        for i in 0..positions.len() {
            for j in i + 1..positions.len() {
                let (x1, y1) = positions[i];
                let (x2, y2) = positions[j];

                // Internal antinode 1
                let num_x = 2 * x1 + x2;
                let num_y = 2 * y1 + y2;
                if num_x % 3 == 0 && num_y % 3 == 0 {
                    let cx = num_x / 3;
                    let cy = num_y / 3;
                    if cx >= 0 && cx < width && cy >= 0 && cy < height {
                        antinodes_part1.insert((cx, cy));
                    }
                }

                // Internal antinode 2
                let num_x = x1 + 2 * x2;
                let num_y = y1 + 2 * y2;
                if num_x % 3 == 0 && num_y % 3 == 0 {
                    let cx = num_x / 3;
                    let cy = num_y / 3;
                    if cx >= 0 && cx < width && cy >= 0 && cy < height {
                        antinodes_part1.insert((cx, cy));
                    }
                }

                // External antinode 1
                let cx = 2 * x1 - x2;
                let cy = 2 * y1 - y2;
                if cx >= 0 && cx < width && cy >= 0 && cy < height {
                    antinodes_part1.insert((cx, cy));
                }

                // External antinode 2
                let cx = 2 * x2 - x1;
                let cy = 2 * y2 - y1;
                if cx >= 0 && cx < width && cy >= 0 && cy < height {
                    antinodes_part1.insert((cx, cy));
                }
            }
        }
    }

    // Part Two
    let mut antinodes_part2 = HashSet::new();

    // For each frequency
    for positions in antennas.values() {
        if positions.len() < 2 {
            continue; // Only one antenna of this frequency
        }

        // Include the antenna positions themselves
        for &(x, y) in positions {
            antinodes_part2.insert((x, y));
        }

        // For each pair of antennas
        for i in 0..positions.len() {
            for j in i + 1..positions.len() {
                let (x1, y1) = positions[i];
                let (x2, y2) = positions[j];

                let dx = x2 - x1;
                let dy = y2 - y1;

                let gcd = gcd(dx, dy);
                let step_x = dx / gcd;
                let step_y = dy / gcd;

                // Backwards
                let mut x = x1 - step_x;
                let mut y = y1 - step_y;
                while x >= 0 && x < width && y >= 0 && y < height {
                    antinodes_part2.insert((x, y));
                    x -= step_x;
                    y -= step_y;
                }

                // Forwards
                let mut x = x1 + step_x;
                let mut y = y1 + step_y;
                while x >= 0 && x < width && y >= 0 && y < height {
                    antinodes_part2.insert((x, y));
                    x += step_x;
                    y += step_y;
                }
            }
        }
    }

    // Output the number of unique antinode positions
    println!("Part One: {}", antinodes_part1.len());
    println!("Part Two: {}", antinodes_part2.len());
}