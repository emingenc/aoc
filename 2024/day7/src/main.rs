use std::collections::HashMap;
use std::fs;
use std::io;

#[derive(Debug)]
struct Equation {
    test_value: i64,
    numbers: Vec<i64>,
}

#[derive(Clone, Copy, Debug)]
enum Operator {
    Add,
    Multiply,
    Concatenate,
}

// Parse a line of input into an Equation struct
fn parse_equation(line: &str) -> Option<Equation> {
    let mut parts = line.split(':');
    let test_value = parts.next()?.trim().parse().ok()?;
    let numbers = parts
        .next()?
        .trim()
        .split_whitespace()
        .filter_map(|n| n.parse().ok())
        .collect();

    Some(Equation { test_value, numbers })
}

// Evaluate the expression with the given operators
fn evaluate(numbers: &[i64], operators: &[Operator]) -> i64 {
    let mut result = numbers[0];
    for (i, &op) in operators.iter().enumerate() {
        match op {
            Operator::Add => result += numbers[i + 1],
            Operator::Multiply => result *= numbers[i + 1],
            Operator::Concatenate => {
                // Concatenate the numbers as strings
                let concat = format!("{}{}", result, numbers[i + 1]);
                result = concat.parse().unwrap_or(0);
            }
        }
    }
    result
}

// Generate all possible operator combinations using memoization
fn can_make_value(
    equation: &Equation,
    operators_list: &[Operator],
    memo: &mut HashMap<(usize, i64), bool>,
) -> bool {
    let key = (equation.numbers.len(), equation.test_value);
    if let Some(&result) = memo.get(&key) {
        return result;
    }

    let operators_needed = equation.numbers.len() - 1;
    let total_combinations = operators_list.len().pow(operators_needed as u32);

    for i in 0..total_combinations {
        let mut operators = Vec::with_capacity(operators_needed);
        let mut n = i;
        // Generate operator combinations
        for _ in 0..operators_needed {
            let op_index = n % operators_list.len();
            operators.push(operators_list[op_index]);
            n /= operators_list.len();
        }
        // Check if the evaluation matches the test value
        if evaluate(&equation.numbers, &operators) == equation.test_value {
            memo.insert(key, true);
            return true;
        }
    }
    memo.insert(key, false);
    false
}

fn solve(input: &str, operators_list: &[Operator]) -> i64 {
    let mut memo = HashMap::new();
    input
        .lines()
        .filter_map(parse_equation)
        .filter(|eq| can_make_value(eq, operators_list, &mut memo))
        .map(|eq| eq.test_value)
        .sum()
}

fn main() -> io::Result<()> {
    let input = fs::read_to_string("input.txt")?;

    // Part 1: Using Add and Multiply operators
    let part1_operators = &[Operator::Add, Operator::Multiply];
    let part1_result = solve(&input, part1_operators);
    println!("Part 1: {}", part1_result);

    // Part 2: Including the Concatenate operator
    let part2_operators = &[Operator::Add, Operator::Multiply, Operator::Concatenate];
    let part2_result = solve(&input, part2_operators);
    println!("Part 2: {}", part2_result);

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    const TEST_INPUT: &str = "\
190: 10 19
3267: 81 40 27
83: 17 5
156: 15 6
7290: 6 8 6 15
161011: 16 10 13
192: 17 8 14
21037: 9 7 18 13
292: 11 6 16 20";

    #[test]
    fn test_part1_example() {
        let operators = &[Operator::Add, Operator::Multiply];
        assert_eq!(solve(TEST_INPUT, operators), 3749);
    }

    #[test]
    fn test_part2_example() {
        let operators = &[Operator::Add, Operator::Multiply, Operator::Concatenate];
        assert_eq!(solve(TEST_INPUT, operators), 11387);
    }

    #[test]
    fn test_concatenate_operator() {
        let numbers = vec![15, 6];
        let operators = vec![Operator::Concatenate];
        assert_eq!(evaluate(&numbers, &operators), 156);
    }
}