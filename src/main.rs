use std::env;
use std::sync::atomic::{AtomicU64, Ordering};
use std::time::{SystemTime, UNIX_EPOCH};

// Embed the external quotes and constants directly into the binary at compile time
// with zero runtime file I/O.
include!("quotes.rs");

/// Raw embedded source of `quotes.rs` baked into the binary's read-only data segment.
pub const EMBEDDED_QUOTES_RAW: &str = include_str!("quotes.rs");

/// Represents a simple civil calendar date.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct SimpleDate {
    pub year: i32,
    pub month: u32,
    pub day: u32,
}

/// Converts a Unix timestamp (seconds since 1970-01-01 UTC) to a `SimpleDate`
/// using Howard Hinnant's civil-from-days algorithm.
pub fn date_from_unix_secs(secs: u64) -> SimpleDate {
    let z = (secs / SECS_PER_DAY) as i64 + CIVIL_EPOCH_OFFSET_DAYS;
    let era = if z >= 0 { z } else { z - 146_096 } / 146_097;
    let doe = (z - era * 146_097) as u32;
    let yoe = (doe - doe / 1460 + doe / 36524 - doe / 146_096) / 365;
    let y = yoe as i64 + era * 400;
    let doy = doe - (365 * yoe + yoe / 4 - yoe / 100);
    let mp = (5 * doy + 2) / 153;
    let d = doy - (153 * mp + 2) / 5 + 1;
    let m = if mp < 10 { mp + 3 } else { mp - 9 };
    let year = if m <= 2 { y + 1 } else { y } as i32;
    SimpleDate {
        year,
        month: m,
        day: d,
    }
}

/// Parses a `YYYY-MM-DD` string into a `SimpleDate`.
pub fn parse_date(input: &str) -> Option<SimpleDate> {
    let mut parts = input.split('-');
    let year: i32 = parts.next()?.parse().ok()?;
    let month: u32 = parts.next()?.parse().ok()?;
    let day: u32 = parts.next()?.parse().ok()?;
    if !(1..=12).contains(&month) || !(1..=31).contains(&day) {
        return None;
    }
    Some(SimpleDate { year, month, day })
}

/// Returns the day of the week (0 = Sunday, 1 = Monday, ..., 4 = Thursday, ..., 6 = Saturday)
/// using Sakamoto's algorithm.
pub fn day_of_week(year: i32, month: u32, day: u32) -> u32 {
    let y = if month < 3 { year - 1 } else { year };
    ((y + y / 4 - y / 100 + y / 400 + SAKAMOTO_MONTH_OFFSETS[(month - 1) as usize] + day as i32)
        .rem_euclid(7)) as u32
}

/// Computes the Nth occurrence (1-indexed) of `target_dow` (0 = Sun .. 6 = Sat) in a given month.
pub fn nth_weekday_of_month(year: i32, month: u32, target_dow: u32, n: u32) -> u32 {
    let first_dow = day_of_week(year, month, 1);
    let offset = (target_dow + 7 - first_dow) % 7;
    1 + offset + (n - 1) * 7
}

/// Computes the last occurrence of `target_dow` in a 31-day month (such as May for Memorial Day).
pub fn last_weekday_of_31_day_month(year: i32, month: u32, target_dow: u32) -> u32 {
    let dow_31 = day_of_week(year, month, 31);
    let offset = (dow_31 + 7 - target_dow) % 7;
    31 - offset
}

/// Computes the day of November on which US Thanksgiving falls (the 4th Thursday of November)
/// for the given year.
pub fn thanksgiving_day_of_november(year: i32) -> u32 {
    nth_weekday_of_month(year, 11, 4, 4)
}

/// Returns the first full day of Hanukkah (25 Kislev) when it falls in December (or late Nov mapped to Dec 1).
pub fn hanukkah_start_day_in_december(year: i32) -> u32 {
    HANUKKAH_START_DAYS
        .iter()
        .find_map(|&(y, day)| if y == year { Some(day) } else { None })
        .unwrap_or(DEFAULT_HANUKKAH_START_DAY)
}

/// Metadata and calculations for a month's featured holiday.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct HolidayInfo {
    pub month_name: &'static str,
    pub holiday_name: &'static str,
    pub greeting_prefix: &'static str,
    pub greetings: &'static [&'static str],
    pub emoji: &'static str,
    pub vibe: &'static str,
    pub start_day: u32,
    pub duration_days: u32,
    pub celebration_banner: &'static str,
    pub post_holiday_note: &'static str,
}

/// Returns the `HolidayInfo` for a given year and month (1..=12) using the embedded `MONTHLY_HOLIDAY_DATA`.
pub fn holiday_for_month(year: i32, month: u32) -> HolidayInfo {
    let idx = if (1..=12).contains(&month) {
        (month - 1) as usize
    } else {
        11
    };
    let data = MONTHLY_HOLIDAY_DATA[idx];
    let start_day = data.fixed_start_day.unwrap_or_else(|| match month {
        5 => last_weekday_of_31_day_month(year, 5, 1),
        8 => nth_weekday_of_month(year, 8, 0, 1),
        9 => nth_weekday_of_month(year, 9, 1, 1),
        11 => thanksgiving_day_of_november(year),
        _ => hanukkah_start_day_in_december(year),
    });

    HolidayInfo {
        month_name: data.month_name,
        holiday_name: data.holiday_name,
        greeting_prefix: data.greeting_prefix,
        greetings: data.greetings,
        emoji: data.emoji,
        vibe: data.vibe,
        start_day,
        duration_days: data.duration_days,
        celebration_banner: data.celebration_banner,
        post_holiday_note: data.post_holiday_note,
    }
}

/// Returns a festive holiday dish/treat recommendation for the given month and day.
pub fn dish_of_the_day_for_month(month: u32, day: u32) -> &'static str {
    let idx = if (1..=12).contains(&month) {
        (month - 1) as usize
    } else {
        11
    };
    let list = MONTHLY_DISHES[idx];
    list[(day as usize) % list.len()]
}

/// Returns a festive Thanksgiving dish recommendation based on the day of the month.
pub fn dish_of_the_day(day: u32) -> &'static str {
    dish_of_the_day_for_month(11, day)
}

/// Helper to build dynamic speech bubble lines around a greeting of any length.
fn bubble_parts(greeting: &str, min_inner_width: usize) -> (String, String, String, String) {
    let char_len = greeting.chars().count();
    let inner_width = char_len.max(min_inner_width);
    let pad = inner_width - char_len;
    let top_bar = "_".repeat(inner_width + 2);
    let empty_row = " ".repeat(inner_width + 2);
    let padded_greeting = format!("{greeting}{}", " ".repeat(pad));
    let bottom_bar = "_".repeat(inner_width + 2);
    (top_bar, empty_row, padded_greeting, bottom_bar)
}

/// Returns ASCII art of a festive Thanksgiving turkey.
pub fn ascii_turkey(greeting: &str) -> String {
    let (top_bar, empty_row, padded, bottom_bar) = bubble_parts(greeting, 28);
    format!(
        r#"   .--.
  / ,. \   🍂  {top_bar}
 | (  ) |     /{empty_row}\
  \ `' /  .__|  {padded} |
   `--' .`    \{bottom_bar}/
   /  \/  \  _.-.
  |   ||   |/    \   /\  /\
  |   ||   |      | /  \/  \
   \  ||  /       |/   /\   \
    `._.'\        /   /  \   |
      ||  `-.__.-'   /   /   /
     _||_           `--'`--'
    '--'--'  🦃 Gobble gobble!"#
    )
}

/// Returns ASCII art of a festive Hanukkah Menorah (Chanukiah).
pub fn ascii_menorah(greeting: &str) -> String {
    let (top_bar, empty_row, padded, bottom_bar) = bubble_parts(greeting, 34);
    format!(
        r#"           🕯️
     🕯️ 🕯️ 🕯️ 🕯️ || 🕯️ 🕯️ 🕯️ 🕯️   ✨  {top_bar}
     || || || || || || || || ||     /{empty_row}\
     \  \  \  \  ||  /  /  /  /  __|  {padded} |
      \  \  \  \ || /  /  /  /     \{bottom_bar}/
       '--.____.\||/.____.--'
                 ||
                _||_
               /____\  🕎 Chag Hanukkah Sameach!"#
    )
}

/// Returns ASCII art of New Year's fireworks and toast.
pub fn ascii_new_years(greeting: &str) -> String {
    let (top_bar, empty_row, padded, bottom_bar) = bubble_parts(greeting, 34);
    format!(
        r#"      .* *.               🎆  {top_bar}
    *  \ /  *   .* *.        /{empty_row}\
   * -- * -- * *  *  *    __|  {padded} |
    *  / \  *   '* *'       \{bottom_bar}/
      '* *'       |
        |      \~/ \~/
       _|_      |   |   🥂 Cheers to the New Year!"#
    )
}

/// Returns seasonal ASCII art for any month of the year.
pub fn ascii_seasonal(month: u32, greeting: &str) -> String {
    match month {
        11 => ascii_turkey(greeting),
        12 => ascii_menorah(greeting),
        1 => ascii_new_years(greeting),
        _ => {
            let info = holiday_for_month(2026, month);
            let (top_bar, empty_row, padded, bottom_bar) = bubble_parts(greeting, 34);
            format!(
                r#"   .~~~~.
  /      \  {}  {top_bar}
 |  ^  ^  |    /{empty_row}\
 |  \__/  | __|  {padded} |
  \      /     \{bottom_bar}/
   `----'   {}Celebrating {}!"#,
                info.emoji, info.emoji, info.holiday_name
            )
        }
    }
}

/// Generates a random `usize` using high-resolution system time, process ID, stack ASLR,
/// and an atomic counter mixed via SplitMix64.
pub fn random_index(len: usize) -> usize {
    if len <= 1 {
        return 0;
    }
    static COUNTER: AtomicU64 = AtomicU64::new(0);
    let seq = COUNTER.fetch_add(1, Ordering::Relaxed);
    let nanos = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .map(|d| d.as_nanos() as u64)
        .unwrap_or(0);
    let pid = std::process::id() as u64;
    let stack_addr = (&nanos as *const u64 as usize) as u64;

    let mut z = nanos
        .wrapping_add(pid.rotate_left(17))
        .wrapping_add(stack_addr.rotate_right(11))
        .wrapping_add(seq.wrapping_mul(SPLITMIX_GAMMA));
    z = (z ^ (z >> 30)).wrapping_mul(SPLITMIX_MUL1);
    z = (z ^ (z >> 27)).wrapping_mul(SPLITMIX_MUL2);
    z = z ^ (z >> 31);
    (z as usize) % len
}

/// Returns the slice of specialized greeting templates for a given month (1..=12).
pub fn greetings_for_month(month: u32) -> &'static [&'static str] {
    holiday_for_month(2026, month).greetings
}

/// Formats a specific specialized greeting (selected by `index`) for the given target name and month (1..=12).
pub fn seasonal_greet_with_index(name: &str, month: u32, index: usize) -> String {
    let trimmed = name.trim();
    let target = if trimmed.is_empty() { "world" } else { trimmed };
    let templates = greetings_for_month(month);
    let template = templates[index % templates.len()];
    template.replace("{name}", target)
}

/// Randomly picks and formats one of the specialized greetings for the given target name and month (1..=12).
pub fn seasonal_greet(name: &str, month: u32) -> String {
    let templates = greetings_for_month(month);
    let idx = random_index(templates.len());
    seasonal_greet_with_index(name, month, idx)
}

/// Formats a random seasonal greeting message for the given target name using the current month.
pub fn greet(name: &str) -> String {
    seasonal_greet(name, current_date().month)
}

/// Generates a seasonal holiday status block for the given date's month.
pub fn seasonal_message(date: SimpleDate, _force_feast: bool) -> Option<String> {
    let info = holiday_for_month(date.year, date.month);
    let mut lines = Vec::new();

    lines.push(format!(
        "{} Happy {} ({:04}-{:02}-{:02})! {} {}",
        info.emoji, info.month_name, date.year, date.month, date.day, info.vibe, info.emoji
    ));

    let end_day = info.start_day + info.duration_days - 1;

    if date.day < info.start_day {
        let days_left = info.start_day - date.day;
        let day_word = if days_left == 1 { "day" } else { "days" };
        let short_month = &info.month_name[..3];
        if date.month == 11 {
            lines.push(format!(
                "🦃 Countdown: {days_left} {day_word} until {} ({short_month} {}, {})!",
                info.holiday_name, info.start_day, date.year
            ));
        } else {
            lines.push(format!(
                "{} Countdown: {days_left} {day_word} until {} ({short_month} {}, {})!",
                info.emoji, info.holiday_name, info.start_day, date.year
            ));
        }
    } else if date.day <= end_day {
        if info.duration_days > 1 {
            let night_num = date.day - info.start_day + 1;
            lines.push(format!(
                "{} (Day {night_num} of {}!)",
                info.celebration_banner, info.duration_days
            ));
        } else {
            lines.push(info.celebration_banner.to_string());
        }
    } else {
        let days_since = date.day - end_day;
        if date.month == 11 {
            lines.push(format!(
                "🥪 Thanksgiving was {days_since} day(s) ago — {}",
                info.post_holiday_note
            ));
        } else {
            lines.push(format!(
                "{} {} was {days_since} day(s) ago — {}",
                info.emoji, info.holiday_name, info.post_holiday_note
            ));
        }
    }

    lines.push(format!(
        "🍽️  Featured {} Treat: {}",
        info.holiday_name,
        dish_of_the_day_for_month(date.month, date.day)
    ));

    Some(lines.join("\n"))
}

fn current_date() -> SimpleDate {
    let secs = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .map(|d| d.as_secs())
        .unwrap_or(0);
    date_from_unix_secs(secs)
}

fn print_usage(program: &str) {
    println!("Usage: {program} [OPTIONS] [NAME...]");
    println!();
    println!("Prints a randomly chosen specialized holiday greeting tailored to every month of the year!");
    println!();
    println!("Options:");
    println!("  -h, --help              Show this help message");
    println!("  -t, --turkey            Force the ASCII art Thanksgiving turkey");
    println!("  -f, --feast             Show the holiday countdown & featured dish");
    println!("  -m, --month MONTH       Override the month (1-12, e.g. --month 12 for Hanukkah)");
    println!("      --date YYYY-MM-DD   Override the full date (e.g. --date 2026-12-05)");
}

fn main() {
    let mut args = env::args();
    let program = args.next().unwrap_or_else(|| "hello".to_string());

    let mut force_turkey = false;
    let mut force_feast = false;
    let mut custom_date: Option<SimpleDate> = None;
    let mut custom_month: Option<u32> = None;
    let mut name_parts: Vec<String> = Vec::new();

    while let Some(arg) = args.next() {
        match arg.as_str() {
            "-h" | "--help" => {
                print_usage(&program);
                return;
            }
            "-t" | "--turkey" => force_turkey = true,
            "-f" | "--feast" => force_feast = true,
            "-m" | "--month" => {
                if let Some(val) = args.next() {
                    if let Ok(m) = val.parse::<u32>() {
                        if (1..=12).contains(&m) {
                            custom_month = Some(m);
                        }
                    }
                }
            }
            "--date" => {
                if let Some(val) = args.next() {
                    custom_date = parse_date(&val);
                }
            }
            _ => name_parts.push(arg),
        }
    }

    let name = if name_parts.is_empty() {
        "world".to_string()
    } else {
        name_parts.join(" ")
    };

    let mut today = custom_date.unwrap_or_else(current_date);
    if let Some(m) = custom_month {
        today.month = m;
        today.day = 1;
    }

    let greeting = seasonal_greet(&name, today.month);

    if force_turkey {
        println!("{}", ascii_turkey(&greeting));
    } else {
        println!("{}", ascii_seasonal(today.month, &greeting));
    }

    if let Some(seasonal) = seasonal_message(today, force_feast) {
        println!("\n{seasonal}");
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn embedded_quotes_file_is_baked_into_binary() {
        assert!(EMBEDDED_QUOTES_RAW.contains("MONTHLY_HOLIDAY_DATA"));
        assert!(EMBEDDED_QUOTES_RAW.contains("NOV_GREETINGS"));
        assert_eq!(MONTHLY_HOLIDAY_DATA.len(), 12);
    }

    #[test]
    fn seasonal_greets_thanksgiving_in_november() {
        assert_eq!(
            seasonal_greet_with_index("world", 11, 0),
            "Happy Thanksgiving, world!"
        );
        let random_nov = seasonal_greet("world", 11);
        let expected_nov: Vec<String> = greetings_for_month(11)
            .iter()
            .map(|t| t.replace("{name}", "world"))
            .collect();
        assert!(expected_nov.contains(&random_nov));
    }

    #[test]
    fn seasonal_greets_hanukkah_in_december() {
        assert_eq!(
            seasonal_greet_with_index("Hannah", 12, 0),
            "Happy Hanukkah, Hannah!"
        );
        let random_dec = seasonal_greet("Hannah", 12);
        let expected_dec: Vec<String> = greetings_for_month(12)
            .iter()
            .map(|t| t.replace("{name}", "Hannah"))
            .collect();
        assert!(expected_dec.contains(&random_dec));
    }

    #[test]
    fn seasonal_greets_new_year_in_january() {
        assert_eq!(
            seasonal_greet_with_index("Rust", 1, 0),
            "Happy New Year, Rust!"
        );
        let random_jan = seasonal_greet("Rust", 1);
        let expected_jan: Vec<String> = greetings_for_month(1)
            .iter()
            .map(|t| t.replace("{name}", "Rust"))
            .collect();
        assert!(expected_jan.contains(&random_jan));
    }

    #[test]
    fn falls_back_on_empty_input() {
        let g = seasonal_greet("   ", 11);
        assert!(g.contains("world"));
    }

    #[test]
    fn each_holiday_has_multiple_specialized_greetings_and_random_picks_multiple() {
        for month in 1..=12 {
            let info = holiday_for_month(2026, month);
            assert!(!info.holiday_name.is_empty());
            assert!(
                info.greetings.len() >= 5,
                "Month {month} should have at least 5 specialized greetings"
            );

            let valid_for_month: Vec<String> = info
                .greetings
                .iter()
                .map(|t| t.replace("{name}", "friend"))
                .collect();

            let mut seen = std::collections::HashSet::new();
            for _ in 0..30 {
                let g = seasonal_greet("friend", month);
                assert!(
                    valid_for_month.contains(&g),
                    "Generated greeting '{g}' must belong to month {month}"
                );
                seen.insert(g);
            }
            assert!(
                seen.len() > 1,
                "Random selection for month {month} should pick across multiple greetings"
            );
        }
    }

    #[test]
    fn computes_thanksgiving_2026_correctly() {
        // In 2026, Nov 1 is a Sunday, so the 4th Thursday is Nov 26.
        assert_eq!(thanksgiving_day_of_november(2026), 26);
    }

    #[test]
    fn november_6_2026_countdown_is_20_days() {
        let nov6 = SimpleDate {
            year: 2026,
            month: 11,
            day: 6,
        };
        let msg = seasonal_message(nov6, false).expect("Should produce November message");
        assert!(msg.contains("20 days until Thanksgiving (Nov 26, 2026)"));
    }

    #[test]
    fn thanksgiving_day_message_celebrates() {
        let tday = SimpleDate {
            year: 2026,
            month: 11,
            day: 26,
        };
        let msg = seasonal_message(tday, false).expect("Should produce Thanksgiving message");
        assert!(msg.contains("HAPPY THANKSGIVING!"));
    }

    #[test]
    fn december_hanukkah_countdown_and_celebration() {
        let dec1 = SimpleDate {
            year: 2026,
            month: 12,
            day: 1,
        };
        let msg_countdown = seasonal_message(dec1, false).expect("Should produce December message");
        assert!(msg_countdown.contains("4 days until Hanukkah (Dec 5, 2026)"));

        let dec6 = SimpleDate {
            year: 2026,
            month: 12,
            day: 6,
        };
        let msg_celebrate = seasonal_message(dec6, false).expect("Should produce Hanukkah message");
        assert!(msg_celebrate.contains("CHAG HANUKKAH SAMEACH!"));
        assert!(msg_celebrate.contains("Day 2 of 8!"));
    }

    #[test]
    fn january_new_years_day_celebrates() {
        let jan1 = SimpleDate {
            year: 2027,
            month: 1,
            day: 1,
        };
        let msg = seasonal_message(jan1, false).expect("Should produce January message");
        assert!(msg.contains("HAPPY NEW YEAR!"));
    }
}


