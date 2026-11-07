# agy — `hello` in Rust 🦃🕎🎆

A lightweight command-line greeting utility written in Rust (2024 edition) with year-round **Monthly Holiday Support** and automatic **Seasonal Greetings**!

## Overview

`hello` prints a festive, seasonal greeting to standard output tailored to the current month (or any custom date/month you choose), randomly picking from a curated set of 6 specialized greetings for that month's holiday on every run:
- Running `hello` with no arguments greets `world` with a randomly chosen specialized greeting for the current month's holiday (e.g., `"Gobble gobble, world — Happy Turkey Day!"` or `"Happy Thanksgiving, world!"` in November).
- Passing one or more arguments greets the specified name (e.g., `hello Antigravity`).
- **Year-Round Monthly Holiday Support (6 Specialized Greetings Each)**:
  - **January** — **New Year's Day** (e.g., `Happy New Year, <name>!`, `Raise a glass to fresh beginnings, <name>!`) with fireworks & champagne toast ASCII art 🎆🥂
  - **February** — **Valentine's Day** (e.g., `Happy Valentine's Day, <name>!`, `You're sweeter than a box of truffles, <name>!`) 💘🍫
  - **March** — **St. Patrick's Day** (e.g., `Happy St. Patrick's Day, <name>!`, `May the luck of the Irish be with you today, <name>!`) ☘️🌈
  - **April** — **Earth Day** (e.g., `Happy Earth Day, <name>!`, `Keep calm and love our planet — Happy Earth Day, <name>!`) 🌍🌱
  - **May** — **Memorial Day** (e.g., `Happy Memorial Day, <name>!`, calculated as the last Monday of May) 🌺☀️
  - **June** — **Juneteenth** (e.g., `Happy Juneteenth, <name>!`, `Celebrating freedom, community, and joy with you, <name>!`) 🌻🎉
  - **July** — **Independence Day** (e.g., `Happy 4th of July, <name>!`, `Have a star-spangled Independence Day, <name>!`) 🎇🗽
  - **August** — **Friendship Day** (e.g., `Happy Friendship Day, <name>!`, calculated as the 1st Sunday of August) 🏖️🤝
  - **September** — **Labor Day** (e.g., `Happy Labor Day, <name>!`, calculated as the 1st Monday of September) 🍎🛠️
  - **October** — **Halloween** (e.g., `Happy Halloween, <name>!`, `Trick or treat — hope your Halloween is spooky and sweet, <name>!`) 🎃👻
  - **November** — **Thanksgiving** (e.g., `Happy Thanksgiving, <name>!`, `Pass the stuffing and cranberry sauce, <name>!`, calculated as the 4th Thursday of November) with an ASCII art turkey 🦃🍂
  - **December** — **Hanukkah** (e.g., `Happy Hanukkah, <name>!`, `Chag Hanukkah Sameach, <name>!`, `Spin the dreidel and pass the crispy latkes, <name>!`, 8-day Festival of Lights celebration) with an ASCII art Menorah 🕎✨
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
- `src/quotes.rs` — External quotes, holiday metadata (`MONTHLY_HOLIDAY_DATA`), specialized greeting pools (`JAN_GREETINGS`..`DEC_GREETINGS`), monthly dishes (`MONTHLY_DISHES`), and calendar/PRNG constants embedded directly into the binary at compile time via `include!("quotes.rs")` and `include_str!("quotes.rs")` with zero runtime file I/O.
- `src/main.rs` — CLI entrypoint, `seasonal_greet()`, `greet()`, `holiday_for_month()`, pure-Rust calendar/weekday/floating-holiday calculations, ASCII art generators (`ascii_turkey`, `ascii_menorah`, `ascii_new_years`, `ascii_seasonal`), and unit tests.
- `.gitignore` — Excludes Cargo's `/target` build directory from version control.

