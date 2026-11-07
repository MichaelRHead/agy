# agy — `hello` in Rust 🦃🕎🎆

A lightweight command-line greeting utility written in Rust (2024 edition) with year-round **Monthly Holiday Support** and automatic **Seasonal Greetings**!

## Overview

`hello` prints a festive, seasonal greeting to standard output tailored to the current month (or any custom date/month you choose):
- Running `hello` with no arguments greets `world` with the current month's holiday greeting (e.g., `Happy Thanksgiving, world!` in November, `Happy Hanukkah, world!` in December, `Happy New Year, world!` in January).
- Passing one or more arguments greets the specified name (e.g., `hello Antigravity` outputs `Happy Thanksgiving, Antigravity!`).
- **Year-Round Monthly Holiday Support**:
  - **January** — **New Year's Day** (`Happy New Year, <name>!`) with fireworks & champagne toast ASCII art 🎆🥂
  - **February** — **Valentine's Day** (`Happy Valentine's Day, <name>!`) 💘🍫
  - **March** — **St. Patrick's Day** (`Happy St. Patrick's Day, <name>!`) ☘️🌈
  - **April** — **Earth Day** (`Happy Earth Day, <name>!`) 🌍🌱
  - **May** — **Memorial Day** (`Happy Memorial Day, <name>!`, calculated as the last Monday of May) 🌺☀️
  - **June** — **Juneteenth** (`Happy Juneteenth, <name>!`) 🌻🎉
  - **July** — **Independence Day** (`Happy 4th of July, <name>!`) 🎇🗽
  - **August** — **Friendship Day** (`Happy Friendship Day, <name>!`, calculated as the 1st Sunday of August) 🏖️🤝
  - **September** — **Labor Day** (`Happy Labor Day, <name>!`, calculated as the 1st Monday of September) 🍎🛠️
  - **October** — **Halloween** (`Happy Halloween, <name>!`) 🎃👻
  - **November** — **Thanksgiving** (`Happy Thanksgiving, <name>!`, calculated as the 4th Thursday of November) with an ASCII art turkey 🦃🍂
  - **December** — **Hanukkah** (`Happy Hanukkah, <name>!`, 8-day Festival of Lights celebration) with an ASCII art Menorah 🕎✨
- Displays a live countdown to the month's holiday, a celebration banner on the holiday (including multi-day tracking for the 8 days of Hanukkah), or a post-holiday message afterward, plus a rotating **Featured Holiday Treat of the Day**.
- Supports `-m` / `--month MONTH`, `--date YYYY-MM-DD`, `-t` / `--turkey`, `-f` / `--feast`, and `-h` / `--help`.

## Usage

### Run with Cargo

```bash
# Default greeting (automatically uses the current month's seasonal greeting & art!)
cargo run

# Custom name greeting
cargo run -- Rustaceans

# Preview December (Hanukkah) or January (New Year's)
cargo run -- --month 12
cargo run -- --date 2026-12-06
cargo run -- --date 2027-01-01

# Time-travel to Thanksgiving Day 2026
cargo run -- --date 2026-11-26 Pilgrim

# Help message
cargo run -- --help
```

### Build and Test

```bash
# Compile a release binary
cargo build --release

# Run the unit test suite
cargo test
```

## Project Structure

- `Cargo.toml` — Package manifest configuring the `hello` binary crate (Rust 2024 edition).
- `src/main.rs` — CLI entrypoint, `seasonal_greet()`, `greet()`, `holiday_for_month()`, pure-Rust calendar/weekday/floating-holiday calculations, ASCII art generators (`ascii_turkey`, `ascii_menorah`, `ascii_new_years`, `ascii_seasonal`), and unit tests.
- `.gitignore` — Excludes Cargo's `/target` build directory from version control.
