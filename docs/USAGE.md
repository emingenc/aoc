**Usage Guide**: How to run and contribute to this Advent of Code repo

- **Project layout**: each year lives under `2024/`, `2025/`, etc. Each day is a folder like `2025/day1/` containing `day1.py`, `day1.md`, and any input/example files.
- **Virtual environment**: create a per-repo venv at `./.venv` and activate it before installing or running Python code.

  Recommended commands (zsh):

  ```zsh
  python3 -m venv .venv
  source .venv/bin/activate
  python -m pip install --upgrade pip setuptools wheel
  pip install -r requirements.txt  # if present
  ```

- **Fetching puzzle inputs**: this repo uses the `advent-of-code-data` package (imported as `aocd`).

  - Install it in the venv: `pip install advent-of-code-data python-dotenv`
  - Supply your AoC session token via `.env` file (recommended) or `export AOC_SESSION=<token>`.
  - Create a `.env` file in the root:
    ```
    AOC_SESSION=your_session_cookie_here
    ```
  - From Python you can call:

    ```py
    from dotenv import load_dotenv
    from aocd import get_data
    
    load_dotenv()
    data = get_data(day=1, year=2025)
    ```

- **Running a day's solution**:

  1. Activate the venv.
  2. Run the script for the day, for example:

     ```zsh
     python 2025/day1/day1.py
     ```

  3. The script should fetch your personal input, print part results, and may include helper functions to parse example input files.

- **Local testing / examples**:

  - Keep example input files in the day's folder (e.g., `example.txt`) and load them in the script if you want reproducible runs without hitting AoC servers.
  - If you want to run multiple days or verify across datasets, consider building a small test harness or using the `aocd` runner (see package docs).

- **Common conventions in this repo**:

  - `dayX.py` — the Python solution scaffold (must contain `main()` so it can be executed).
  - `dayX.md` — puzzle description / notes / instructions.
  - `input.txt` or `example.txt` — local copies of inputs/examples.

- **Security**:

  - Never commit your AoC session token or `~/.config/aocd/token` to version control.
  - `.gitignore` already contains `.venv` entries.

- **Automating setup**:

  - A convenience script `scripts/setup-venv.sh` is included to create the venv and install the `advent-of-code-data` package.

Questions or custom workflows? Open an issue or ask in the repo — I'm happy to add CI, test harness, or a CLI runner.
