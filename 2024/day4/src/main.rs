use std::fs::File;
use std::io::{self, BufRead, BufReader};

fn read_matrix(filename: &str) -> io::Result<Vec<Vec<char>>> {
    let file = File::open(filename)?;
    let reader = BufReader::new(file);
    let matrix: Vec<Vec<char>> = reader
        .lines()
        .filter_map(Result::ok)
        .map(|line| line.chars().collect())
        .collect();
    Ok(matrix)
}

fn search_word(matrix: &Vec<Vec<char>>, word: &str) -> usize {
    let rows = matrix.len();
    let cols = if rows > 0 { matrix[0].len() } else { 0 };
    let word_chars: Vec<char> = word.chars().collect();
    let word_len = word_chars.len();
    let mut total = 0;

    let directions = [
        (-1, 0),  // Up
        (1, 0),   // Down
        (0, -1),  // Left
        (0, 1),   // Right
        (-1, -1), // Up-Left
        (-1, 1),  // Up-Right
        (1, -1),  // Down-Left
        (1, 1),   // Down-Right
    ];

    for row in 0..rows {
        for col in 0..cols {
            for &(dx, dy) in &directions {
                let mut found = true;
                for k in 0..word_len {
                    let new_row = row as isize + dx * k as isize;
                    let new_col = col as isize + dy * k as isize;

                    if new_row < 0
                        || new_row >= rows as isize
                        || new_col < 0
                        || new_col >= cols as isize
                        || matrix[new_row as usize][new_col as usize] != word_chars[k]
                    {
                        found = false;
                        break;
                    }
                }
                if found {
                    total += 1;
                }
            }
        }
    }
    total
}

fn is_mas_sequence(chars: [char; 3]) -> bool {
    chars == ['M', 'A', 'S'] || chars == ['S', 'A', 'M']
}

fn search_x_mas(matrix: &Vec<Vec<char>>) -> usize {
    let rows = matrix.len();
    let cols = if rows > 0 { matrix[0].len() } else { 0 };
    let mut total = 0;

    for r in 1..rows - 1 {
        for c in 1..cols - 1 {
            if matrix[r][c] != 'A' {
                continue;
            }

            let diag1 = [
                matrix[r - 1][c - 1],
                matrix[r][c],
                matrix[r + 1][c + 1],
            ];
            let diag2 = [
                matrix[r - 1][c + 1],
                matrix[r][c],
                matrix[r + 1][c - 1],
            ];

            if is_mas_sequence(diag1) && is_mas_sequence(diag2) {
                total += 1;
            }
        }
    }
    total
}


fn main() -> io::Result<()> {
    let matrix = read_matrix("4.txt")?;
    let total_occurrences = search_word(&matrix, "XMAS");
    let total_mas_occurrences = search_x_mas(&matrix);
    println!("Total occurrences of 'X-MAS': {}", total_mas_occurrences);
    println!("Total occurrences of 'XMAS': {}", total_occurrences);
    Ok(())
}