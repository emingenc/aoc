#!/usr/bin/env python3
"""AoC 2025 Day 2"""
from __future__ import annotations

from dotenv import load_dotenv
from aocd import get_data, submit
from pathlib import Path
from typing import List

load_dotenv()


def parse_input(raw: str) -> List[str]:
    return raw.strip().splitlines()

def is_valid_id(id_str: str) -> bool:
    if len(id_str) % 2 != 0:
        return True
    half = len(id_str) // 2
    return  id_str[:half] != id_str[half:]

# Now, an ID is invalid if it is made only of some sequence of digits repeated at least twice. So, 12341234 (1234 two times), 123123123 (123 three times), 1212121212 (12 four times), and 1111111 (1 seven times) are all invalid IDs.
def new_is_invalid_id(id_str: str) -> bool:
    length = len(id_str)
    for sub_len in range(1, length // 2 + 1):
        if length % sub_len == 0:
            times = length // sub_len
            substring = id_str[:sub_len]
            if substring * times == id_str:
                return True
    return False

def part1(lines: List[str]):
    ids = lines[0].split(",")
    res = []
    for id_str in ids:
        start, end = id_str.split("-")
        for i in range(int(start), int(end) + 1):
            if not is_valid_id(str(i)):
                res.append(int(i))
    
    return sum(res)


def part2(lines: List[str]):
    ids = lines[0].split(",")
    res = []
    for id_str in ids:
        start, end = id_str.split("-")
        for i in range(int(start), int(end) + 1):
            if  new_is_invalid_id(str(i)):
                res.append(int(i))
    
    return sum(res)


def main() -> None:
    # Fetch data
    data = get_data(day=2, year=2025)
    
    # Save locally
    Path("input.txt").write_text(data)
    # data = """11-22,95-115,998-1012,1188511880-1188511890,222220-222224,1698522-1698528,446443-446449,38593856-38593862,565653-565659,824824821-824824827,2121212118-2121212124"""
    lines = parse_input(data)
    
    p1 = part1(lines)
    print("Part 1:", p1)
    # if p1 is not None:
    #     submit(p1, part="a", day=2, year=2025)

    p2 = part2(lines)
    print("Part 2:", p2)
    # if p2 is not None:
    #     submit(p2, part="b", day=2, year=2025)


if __name__ == "__main__":
    main()
