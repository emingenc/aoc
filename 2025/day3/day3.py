#!/usr/bin/env python3
"""AoC 2025 Day 3"""
from __future__ import annotations

from dotenv import load_dotenv
from aocd import get_data, submit
from pathlib import Path
from typing import List

# Load environment variables for AOCD
load_dotenv()


def parse_input(raw: str) -> List[str]:
    """
    Parses the raw input string into a list of lines.
    """
    return raw.strip().splitlines()


def get_digits(line: str) -> List[int]:
    """
    Extracts all digits from a string and returns them as a list of integers.
    """
    return [int(c) for c in line if c.isdigit()]


def find_largest_subsequence(digits: List[int], length: int) -> int:
    """
    Finds the largest number formed by a subsequence of `length` digits
    from the input `digits` list, preserving relative order.
    
    This uses a monotonic stack approach (greedy strategy) to keep the 
    largest possible digits at the front of the sequence.
    """
    # If we need more digits than available, it's impossible (or return what we have)
    if length > len(digits):
        return 0 
        
    # Calculate how many digits we are allowed to remove to achieve the target length
    to_remove = len(digits) - length
    stack = []
    
    for digit in digits:
        # While we can still remove digits, and the current digit is larger
        # than the last one we picked, discard the smaller one to make room
        # for the larger one (greedy choice).
        while to_remove > 0 and stack and stack[-1] < digit:
            stack.pop()
            to_remove -= 1
        stack.append(digit)
        
    # If we didn't remove enough digits (e.g., the sequence was already descending),
    # truncate the end of the stack to fit the desired length.
    while to_remove > 0 and stack:
        stack.pop()
        to_remove -= 1
        
    # Convert the stack of digits back to an integer
    if not stack:
        return 0
        
    return int("".join(map(str, stack)))


def part1(lines: List[str]) -> int:
    """
    Solves Part 1: Find the largest 2-digit number for each line.
    Sum these maximums.
    """
    total_joltage = 0
    for line in lines:
        digits = get_digits(line)
        # We want the largest subsequence of length 2
        max_joltage = find_largest_subsequence(digits, 2)
        total_joltage += max_joltage
        
    return total_joltage


def part2(lines: List[str]) -> int:
    """
    Solves Part 2: Find the largest 12-digit number for each line.
    Sum these maximums.
    """
    total_joltage = 0
    for line in lines:
        digits = get_digits(line)
        # We want the largest subsequence of length 12
        val = find_largest_subsequence(digits, 12)
        total_joltage += val
            
    return total_joltage


def main() -> None:
    """
    Main function to orchestrate data fetching, parsing, and solving.
    """
    # Fetch data from AoC
    data = get_data(day=3, year=2025)
    
    # Save input locally for debugging/reference
    Path("input.txt").write_text(data)
    
    # Parse the input
    lines = parse_input(data)
    
    # Solve Part 1
    p1 = part1(lines)
    print("Part 1:", p1)
    # Uncomment to submit
    # if p1 is not None:
    #     submit(p1, part="a", day=3, year=2025)

    # Solve Part 2
    p2 = part2(lines)
    print("Part 2:", p2)
    # Uncomment to submit
    # if p2 is not None:
    #     submit(p2, part="b", day=3, year=2025)


if __name__ == "__main__":
    main()
