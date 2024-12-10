use std::collections::{HashSet, VecDeque};
use std::fs;

fn main() {
    let input = fs::read_to_string("input.txt").expect("Failed to read input.txt");
    let grid: Vec<Vec<u8>> = input
        .lines()
        .map(|line| line.trim().chars().map(|c| c as u8 - b'0').collect())
        .collect();

    let rows = grid.len();
    let cols = grid[0].len();
    let mut trailheads = Vec::new();
    
    // Find trailheads (height 0)
    for y in 0..rows {
        for x in 0..cols {
            if grid[y][x] == 0 {
                trailheads.push((x, y));
            }
        }
    }

    // Part 1: Count reachable 9s
    let mut part1_score = 0;
    for &start in &trailheads {
        part1_score += count_reachable_nines(&grid, start);
    }
    
    // Part 2: Count distinct paths
    let mut part2_score = 0;
    for &start in &trailheads {
        let mut visited = HashSet::new();
        visited.insert(start);
        part2_score += count_distinct_paths(&grid, start, &mut visited, 0);
    }
    
    println!("Part 1: {}", part1_score);
    println!("Part 2: {}", part2_score);
}

fn count_reachable_nines(grid: &Vec<Vec<u8>>, start: (usize, usize)) -> usize {
    let mut queue = VecDeque::new();
    let mut visited = HashSet::new();
    let mut nines = HashSet::new();
    queue.push_back((start, grid[start.1][start.0]));
    visited.insert(start);

    while let Some(((x, y), height)) = queue.pop_front() {
        if height == 9 {
            nines.insert((x, y));
            continue;
        }

        for &(dx, dy) in &[(-1, 0), (1, 0), (0, -1), (0, 1)] {
            let new_x = (x as i32 + dx) as usize;
            let new_y = (y as i32 + dy) as usize;
            
            if new_x < grid[0].len() && new_y < grid.len() 
               && !visited.contains(&(new_x, new_y))
               && grid[new_y][new_x] == height + 1 {
                visited.insert((new_x, new_y));
                queue.push_back(((new_x, new_y), grid[new_y][new_x]));
            }
        }
    }
    nines.len()
}

fn count_distinct_paths(
    grid: &Vec<Vec<u8>>, 
    pos: (usize, usize), 
    visited: &mut HashSet<(usize, usize)>,
    height: u8
) -> usize {
    if height == 9 {
        return 1;
    }

    let mut paths = 0;
    let (x, y) = pos;

    for &(dx, dy) in &[(-1, 0), (1, 0), (0, -1), (0, 1)] {
        let new_x = (x as i32 + dx) as usize;
        let new_y = (y as i32 + dy) as usize;
        
        if new_x < grid[0].len() && new_y < grid.len() 
           && !visited.contains(&(new_x, new_y))
           && grid[new_y][new_x] == height + 1 {
            visited.insert((new_x, new_y));
            paths += count_distinct_paths(grid, (new_x, new_y), visited, height + 1);
            visited.remove(&(new_x, new_y));
        }
    }
    paths
}