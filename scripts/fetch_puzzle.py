#!/usr/bin/env python3
"""
Fetch AoC puzzle text (prose) and append it to the day's markdown file.
Usage: python scripts/fetch_puzzle.py [year] [day]
"""

import argparse
import sys
import re
from pathlib import Path
from bs4 import BeautifulSoup

try:
    from aocd.models import Puzzle
    from dotenv import load_dotenv
    load_dotenv()
except ImportError:
    sys.exit("Error: dependencies not installed. Run: pip install advent-of-code-data python-dotenv beautifulsoup4")

def main():
    parser = argparse.ArgumentParser(description="Fetch AoC puzzle text")
    parser.add_argument("year", type=int, help="Year (e.g. 2025)")
    parser.add_argument("day", type=int, help="Day (1-25)")
    args = parser.parse_args()

    year = args.year
    day = args.day

    print(f"Fetching puzzle text for {year} Day {day}...")
    
    try:
        puzzle = Puzzle(year=year, day=day)
        # _get_prose() returns the full HTML page or cached content
        # We need to extract the <article class="day-desc"> content
        html_content = puzzle._get_prose()
        
        soup = BeautifulSoup(html_content, "html.parser")
        articles = soup.find_all("article", class_="day-desc")
        
        if not articles:
            print("Error: Could not find puzzle description in response.")
            return

        markdown_text = ""
        for article in articles:
            # Simple HTML to Markdown conversion
            # This is a basic approximation
            text = article.get_text(separator="\n\n")
            markdown_text += text + "\n\n"

        # Clean up some common artifacts if needed
        # (e.g. removing "--- Day X: Title ---" if we already have a title)
        
        # Define path
        base_dir = Path(__file__).parent.parent
        day_dir = base_dir / str(year) / f"day{day}"
        readme_path = day_dir / f"day{day}.md"
        
        if not readme_path.exists():
            print(f"Error: {readme_path} does not exist. Scaffold the day first.")
            return

        current_content = readme_path.read_text()
        
        # Check if we already have the description
        if "The Elves have good news" in current_content: # Heuristic check
             print("Puzzle text seems to be already present.")
             return

        # Append or replace placeholder
        if "<!-- Paste the full puzzle text here -->" in current_content:
             new_content = current_content.replace(
                 "<!-- Paste the full puzzle text here -->\n> (Puzzle text omitted. Copy-paste from the link above if you wish to store it locally.)", 
                 markdown_text
             )
        elif "> **Note**: The full puzzle text is not included due to copyright." in current_content:
             new_content = current_content.replace(
                 "> **Note**: The full puzzle text is not included due to copyright. Please visit the link above to read the story.",
                 markdown_text
             )
        else:
            new_content = current_content + "\n\n## Puzzle Description\n\n" + markdown_text

        readme_path.write_text(new_content)
        print(f"Updated {readme_path} with puzzle text.")

    except Exception as e:
        print(f"Error fetching puzzle: {e}")

if __name__ == "__main__":
    main()
