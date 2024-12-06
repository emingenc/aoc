use std::collections::HashSet;
use std::fs::File;
use std::io::{self, BufRead, BufReader};

#[derive(PartialEq, Eq, Hash, Clone, Copy)]
enum Direction {
    Up,
    Right,
    Down,
    Left,
}

impl Direction {
    fn turn_right(&self) -> Direction {
        match self {
            Direction::Up => Direction::Right,
            Direction::Right => Direction::Down,
            Direction::Down => Direction::Left,
            Direction::Left => Direction::Up,
        }
    }

    fn move_forward(&self, position: (usize, usize)) -> Option<(usize, usize)> {
        let (x, y) = position;
        match self {
            Direction::Up => y.checked_sub(1).map(|ny| (x, ny)),
            Direction::Right => Some((x + 1, y)),
            Direction::Down => Some((x, y + 1)),
            Direction::Left => x.checked_sub(1).map(|nx| (nx, y)),
        }
    }

    fn from_char(c: char) -> Option<Direction> {
        match c {
            '^' => Some(Direction::Up),
            '>' => Some(Direction::Right),
            'v' => Some(Direction::Down),
            '<' => Some(Direction::Left),
            _ => None,
        }
    }
}

#[derive(PartialEq, Eq, Hash, Clone, Copy)]
struct State {
    pos: (usize, usize),
    dir: Direction,
}

fn is_loop(map: &Vec<Vec<char>>, start_pos: (usize, usize), start_dir: Direction) -> bool {
    let mut visited: HashSet<State> = HashSet::new();
    let mut current_pos = start_pos;
    let mut current_dir = start_dir;
    let mut steps = 0;
    const MAX_STEPS: usize = 10000; // Increased limit
    
    while steps < MAX_STEPS {
        steps += 1;
        
        let state = State {
            pos: current_pos,
            dir: current_dir,
        };
        
        // If we've seen this exact state before, it's a loop
        if !visited.insert(state) {
            return true;
        }
        
        // Try to move forward
        if let Some(next_pos) = current_dir.move_forward(current_pos) {
            // Check if we're still in bounds
            if next_pos.0 >= map[0].len() || next_pos.1 >= map.len() {
                return false;
            }
            
            // Check for obstacle and move/turn accordingly
            if map[next_pos.1][next_pos.0] == '#' {
                current_dir = current_dir.turn_right();
            } else {
                current_pos = next_pos;
            }
        } else {
            return false;
        }
    }
    
    false
}

fn count_loop_positions(map: &Vec<Vec<char>>, guard_pos: (usize, usize), start_dir: Direction) -> usize {
    let mut count = 0;
    
    for y in 0..map.len() {
        for x in 0..map[0].len() {
            // Skip if not empty space or guard position
            if map[y][x] != '.' || (x, y) == guard_pos {
                continue;
            }
            
            // Create test map with new obstacle
            let mut test_map = map.clone();
            test_map[y][x] = '#';
            
            if is_loop(&test_map, guard_pos, start_dir) {
                count += 1;
                println!("Found loop at ({}, {})", x, y);
            }
        }
    }
    
    count
}

fn solve_part1(map: &Vec<Vec<char>>, start_pos: (usize, usize), start_dir: Direction) -> usize {
    let mut visited: HashSet<(usize, usize)> = HashSet::new();
    let mut current_pos = start_pos;
    let mut current_dir = start_dir;

    loop {
        visited.insert(current_pos);
        
        if let Some(next_pos) = current_dir.move_forward(current_pos) {
            if next_pos.0 >= map[0].len() || next_pos.1 >= map.len() {
                break;
            }
            
            if map[next_pos.1][next_pos.0] == '#' {
                current_dir = current_dir.turn_right();
            } else {
                current_pos = next_pos;
            }
        } else {
            break;
        }
    }
    
    visited.len()
}

fn main() -> io::Result<()> {
    let file = File::open("input.txt")?;
    let reader = BufReader::new(file);
    let mut map: Vec<Vec<char>> = Vec::new();
    let mut guard_pos = (0, 0);
    let mut direction = Direction::Up;
    
    // Read map (unchanged)
    for (y, line) in reader.lines().enumerate() {
        let line = line?;
        let row: Vec<char> = line.chars().collect();
        for (x, &c) in row.iter().enumerate() {
            if let Some(dir) = Direction::from_char(c) {
                guard_pos = (x, y);
                direction = dir;
            }
        }
        map.push(row);
    }
    
    let part1 = solve_part1(&map, guard_pos, direction);
    let part2 = count_loop_positions(&map, guard_pos, direction);
    
    println!("Part 1: {}", part1);
    println!("Part 2: {}", part2);
    
    Ok(())
}