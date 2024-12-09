use std::fs;

fn main() {
    // Read the disk map from "input.txt"
    let input = fs::read_to_string("input.txt").expect("Failed to read input.txt");
    let disk_map = input.trim();

    // Parse the disk map into lengths of files and free spaces
    let mut lengths = Vec::new();
    for c in disk_map.chars() {
        let n = c.to_digit(10).expect("Invalid character in input") as usize;
        lengths.push(n);
    }

    // Part One

    // Build the initial disk blocks
    let mut blocks = Vec::new();
    let mut file_id = 0;
    let mut is_file = true;
    for length in &lengths {
        if is_file {
            for _ in 0..*length {
                blocks.push(Some(file_id));
            }
            file_id += 1;
        } else {
            for _ in 0..*length {
                blocks.push(None);
            }
        }
        is_file = !is_file;
    }

    // Move file blocks one at a time from the end to the leftmost free space
    loop {
        if let Some(free_pos) = blocks.iter().position(|&b| b.is_none()) {
            if let Some(last_file_pos) = blocks.iter().rposition(|&b| b.is_some()) {
                if last_file_pos < free_pos {
                    break;
                }
                blocks[free_pos] = blocks[last_file_pos];
                blocks[last_file_pos] = None;
            } else {
                break;
            }
        } else {
            break;
        }
    }

    // Calculate the filesystem checksum for Part One
    let mut checksum = 0;
    for (position, &block) in blocks.iter().enumerate() {
        if let Some(file_id) = block {
            checksum += position * file_id as usize;
        }
    }

    println!("Part One - Filesystem checksum: {}", checksum);

    // Part Two

    // Rebuild the initial disk blocks and record files
    let mut blocks = Vec::new();
    let mut file_id = 0;
    let mut is_file = true;
    let mut files = Vec::new(); // (file_id, start_pos, length)
    let mut index = 0;
    for length in &lengths {
        if is_file {
            files.push((file_id, index, *length));
            for _ in 0..*length {
                blocks.push(Some(file_id));
                index += 1;
            }
            file_id += 1;
        } else {
            for _ in 0..*length {
                blocks.push(None);
                index += 1;
            }
        }
        is_file = !is_file;
    }

    // Move files in decreasing file ID order
    for &(file_id, orig_pos, length) in files.iter().rev() {
        let mut new_pos = None;
        for i in 0..orig_pos {
            if i + length > orig_pos {
                break;
            }
            if blocks[i..i + length].iter().all(|&b| b.is_none()) {
                new_pos = Some(i);
                break;
            }
        }
        if let Some(pos) = new_pos {
            // Remove the file from its original position
            for i in orig_pos..orig_pos + length {
                blocks[i] = None;
            }
            // Place the file at the new position
            for i in pos..pos + length {
                blocks[i] = Some(file_id);
            }
        }
    }

    // Calculate the filesystem checksum for Part Two
    let mut checksum = 0;
    for (position, &block) in blocks.iter().enumerate() {
        if let Some(file_id) = block {
            checksum += position * file_id as usize;
        }
    }

    println!("Part Two - Filesystem checksum: {}", checksum);
}