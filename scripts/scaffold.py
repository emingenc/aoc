#!/usr/bin/env python3
"""
Scaffold a new Advent of Code day.
Usage: python scripts/scaffold.py [year] [day]
If year/day are not provided, defaults to today (EST).
"""

import argparse
import datetime
import os
import sys
from pathlib import Path

try:
    from aocd import get_data
    from dotenv import load_dotenv
    load_dotenv()
except ImportError:
    sys.exit("Error: dependencies not installed. Run: pip install advent-of-code-data python-dotenv")

TEMPLATE = '''#!/usr/bin/env python3
"""AoC {year} Day {day}"""
from __future__ import annotations

from dotenv import load_dotenv
from aocd import get_data, submit
from pathlib import Path
from typing import List

load_dotenv()


def parse_input(raw: str) -> List[str]:
    return raw.strip().splitlines()


def part1(lines: List[str]):
    # TODO: implement Part 1
    return None


def part2(lines: List[str]):
    # TODO: implement Part 2
    return None


def main() -> None:
    # Fetch data
    data = get_data(day={day}, year={year})
    
    # Save locally
    Path("input.txt").write_text(data)
    
    lines = parse_input(data)
    
    p1 = part1(lines)
    print("Part 1:", p1)
    if p1 is not None:
        submit(p1, part="a", day={day}, year={year})

    p2 = part2(lines)
    print("Part 2:", p2)
    if p2 is not None:
        submit(p2, part="b", day={day}, year={year})


if __name__ == "__main__":
    main()
'''

README_TEMPLATE = '''# Advent of Code {year} — Day {day}

[Original Puzzle](https://adventofcode.com/{year}/day/{day})

## Notes

- Input is saved to `input.txt` when you run the script.
'''

def get_est_date():
    # AoC unlocks at midnight EST (UTC-5)
    utc_now = datetime.datetime.utcnow()
    est_now = utc_now - datetime.timedelta(hours=5)
    return est_now

def main():
    parser = argparse.ArgumentParser(description="Scaffold AoC day")
    parser.add_argument("year", type=int, nargs="?", help="Year (e.g. 2025)")
    parser.add_argument("day", type=int, nargs="?", help="Day (1-25)")
    args = parser.parse_args()

    now = get_est_date()
    year = args.year or now.year
    day = args.day or now.day

    if not (1 <= day <= 25):
        sys.exit(f"Error: Day must be 1-25, got {day}")

    # Define paths
    base_dir = Path(__file__).parent.parent
    day_dir = base_dir / str(year) / f"day{day}"
    script_path = day_dir / f"day{day}.py"
    readme_path = day_dir / f"day{day}.md"
    input_path = day_dir / "input.txt"

    print(f"Scaffolding {year} Day {day} at {day_dir}...")

    # Create directory
    day_dir.mkdir(parents=True, exist_ok=True)

    # Create script if missing
    if not script_path.exists():
        script_content = TEMPLATE.format(year=year, day=day)
        script_path.write_text(script_content)
        script_path.chmod(0o755)
        print(f"Created {script_path}")
    else:
        print(f"Skipped {script_path} (exists)")

    # Create README if missing
    if not readme_path.exists():
        readme_content = README_TEMPLATE.format(year=year, day=day)
        readme_path.write_text(readme_content)
        print(f"Created {readme_path}")
    else:
        print(f"Skipped {readme_path} (exists)")

    # Try to fetch input
    if not input_path.exists():
        print("Fetching input...")
        try:
            data = get_data(day=day, year=year)
            input_path.write_text(data)
            print(f"Saved input to {input_path}")
        except Exception as e:
            print(f"Warning: Could not fetch input: {e}")
            print("Make sure AOC_SESSION is set or you are logged in via `aocd-token`.")
    else:
        print(f"Skipped {input_path} (exists)")

    # Fetch puzzle text
    print("Fetching puzzle text...")
    try:
        import subprocess
        fetch_script = base_dir / "scripts" / "fetch_puzzle.py"
        subprocess.run([sys.executable, str(fetch_script), str(year), str(day)], check=True)
    except Exception as e:
        print(f"Warning: Could not fetch puzzle text: {e}")

    print("Done!")

if __name__ == "__main__":
    main()
