use std::collections::{HashMap, HashSet, VecDeque};
use std::fs::File;
use std::io::{self, BufRead, BufReader};

fn main() -> io::Result<()> {
    // Open the input file
    let file = File::open("input.txt")?;
    let reader = BufReader::new(file);

    // Read lines from the file
    let mut lines = reader.lines();

    // Parse ordering rules
    let mut ordering_rules = Vec::new();
    for line in &mut lines {
        let line = line?;
        let trimmed = line.trim();
        if trimmed.is_empty() {
            break; // Blank line indicates end of ordering rules
        }
        let parts: Vec<&str> = trimmed.split('|').collect();
        if parts.len() == 2 {
            let x = parts[0]
                .parse::<u32>()
                .expect("Invalid number in ordering rules");
            let y = parts[1]
                .parse::<u32>()
                .expect("Invalid number in ordering rules");
            ordering_rules.push((x, y));
        }
    }

    // Parse updates
    let mut updates = Vec::new();
    for line in &mut lines {
        let line = line?;
        let trimmed = line.trim();
        if !trimmed.is_empty() {
            let pages: Vec<u32> = trimmed
                .split(',')
                .map(|s| s.trim().parse::<u32>().expect("Invalid number in updates"))
                .collect();
            updates.push(pages);
        }
    }

    let mut total_part1 = 0;
    let mut total_part2 = 0;

    // Store incorrectly-ordered updates for Part Two
    let mut incorrect_updates = Vec::new();

    for pages in updates {
        // Map page numbers to their positions
        let mut position = HashMap::new();
        for (idx, &page) in pages.iter().enumerate() {
            position.insert(page, idx);
        }

        // Check applicable ordering rules
        let mut valid = true;
        for &(x, y) in &ordering_rules {
            if let (Some(&pos_x), Some(&pos_y)) = (position.get(&x), position.get(&y)) {
                if pos_x >= pos_y {
                    // Rule violated
                    valid = false;
                    break;
                }
            }
            // If one of the pages is not in the update, ignore the rule
        }

        if valid {
            // Update is correctly ordered
            let middle_idx = pages.len() / 2; // For odd lengths, this is the middle index
            total_part1 += pages[middle_idx];
        } else {
            // Collect incorrectly-ordered updates for Part Two
            incorrect_updates.push(pages);
        }
    }

    println!("Part One: Total sum of middle page numbers: {}", total_part1);

    // Part Two: Reorder incorrectly-ordered updates
    for pages in incorrect_updates {
        // Reorder the pages using topological sort
        if let Some(sorted_pages) = topological_sort(&pages, &ordering_rules) {
            // Find the middle page number
            let middle_idx = sorted_pages.len() / 2;
            total_part2 += sorted_pages[middle_idx];
        } else {
            // If no valid ordering exists (shouldn't happen), handle accordingly
            println!("No valid ordering exists for update: {:?}", pages);
        }
    }

    println!("Part Two: Total sum of middle page numbers: {}", total_part2);

    Ok(())
}

// Function to perform topological sort
fn topological_sort(
    pages: &Vec<u32>,
    ordering_rules: &Vec<(u32, u32)>,
) -> Option<Vec<u32>> {
    // Build graph and in-degree map
    let mut graph: HashMap<u32, Vec<u32>> = HashMap::new();
    let mut in_degree: HashMap<u32, usize> = HashMap::new();
    let page_set: HashSet<u32> = pages.iter().cloned().collect();

    // Initialize in-degree of all pages to 0
    for &page in &page_set {
        in_degree.insert(page, 0);
        graph.insert(page, Vec::new());
    }

    // Build the graph and in-degree map based on applicable ordering rules
    for &(x, y) in ordering_rules {
        if page_set.contains(&x) && page_set.contains(&y) {
            // x must come before y
            graph.get_mut(&x).unwrap().push(y);
            *in_degree.get_mut(&y).unwrap() += 1;
        }
    }

    // Use a min-heap or sorted list for deterministic output
    let mut zero_in_degree: Vec<u32> = in_degree
        .iter()
        .filter(|&(_, &deg)| deg == 0)
        .map(|(&page, _)| page)
        .collect();
    zero_in_degree.sort(); // Ensure deterministic ordering

    let mut sorted_pages = Vec::new();

    while !zero_in_degree.is_empty() {
        // Remove a page from zero_in_degree
        let page = zero_in_degree.remove(0);
        sorted_pages.push(page);

        // Decrease in-degree of neighbors
        for &neighbor in &graph[&page] {
            let deg = in_degree.get_mut(&neighbor).unwrap();
            *deg -= 1;
            if *deg == 0 {
                zero_in_degree.push(neighbor);
            }
        }

        zero_in_degree.sort(); // Keep the list sorted for deterministic output
    }

    if sorted_pages.len() == page_set.len() {
        Some(sorted_pages)
    } else {
        // Cycle detected (shouldn't happen in this problem)
        None
    }
}