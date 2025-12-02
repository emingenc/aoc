#!/usr/bin/env python3
"""AoC 2025 Day 1"""
from __future__ import annotations

from dotenv import load_dotenv
from aocd import get_data, submit
from pathlib import Path
from typing import List

load_dotenv()


def parse_input(raw: str) -> List[str]:
    return raw.strip().splitlines()

def get_instruction(line: str) -> int:
    # L, R turn instructions
    turn = line[0]
    value = int(line[1:])
    if turn == 'L':
        return -value
    elif turn == 'R':
        return value
    else:
        raise ValueError(f"Invalid turn instruction: {turn}")




def part1(lines: List[str]):
    curr = 50
    zero_count = 0
    for line in lines:
        instr = get_instruction(line)
        curr = (curr + instr) % 100
        if curr == 0:
            zero_count += 1
    return zero_count
    


def part2(lines: List[str]):
    curr = 50
    zero_count = 0
    for line in lines:
        instr = get_instruction(line)
        target = curr + instr
        wraps = target // 100
        zero_count += abs(wraps)
        curr = target % 100
    return zero_count


def main() -> None:
    # Fetch data
    data = get_data(day=1, year=2025)
    
    # Save locally
    Path("input.txt").write_text(data)
    
    lines = parse_input(data)
    
    p1 = part1(lines)
    print("Part 1:", p1)
    # if p1 is not None:
    #     submit(p1, part="a", day=1, year=2025)

    p2 = part2(lines)
    print("Part 2:", p2)
    # if p2 is not None:
    #     submit(p2, part="b", day=1, year=2025)


if __name__ == "__main__":
    main()
