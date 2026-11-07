use std::env;
use std::time::{SystemTime, UNIX_EPOCH};

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
    let z = (secs / 86_400) as i64 + 719_468;
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
    let t = [0, 3, 2, 5, 0, 3, 5, 1, 4, 6, 2, 4];
    let y = if month < 3 { year - 1 } else { year };
    ((y + y / 4 - y / 100 + y / 400 + t[(month - 1) as usize] + day as i32)
        .rem_euclid(7)) as u32
}

/// Computes the day of November on which US Thanksgiving falls (the 4th Thursday of November)
/// for the given year.
pub fn thanksgiving_day_of_november(year: i32) -> u32 {
    let nov1_dow = day_of_week(year, 11, 1); // 0 = Sun, 4 = Thu
    let offset_to_first_thursday = (4 + 7 - nov1_dow) % 7;
    let first_thursday = 1 + offset_to_first_thursday;
    first_thursday + 21
}

/// Returns a festive Thanksgiving dish recommendation based on the day of the month.
pub fn dish_of_the_day(day: u32) -> &'static str {
    const DISHES: &[&str] = &[
        "Herb-Roasted Turkey with Crispy Skin 🦃",
        "Homemade Cranberry-Orange Relish 🍒",
        "Creamy Garlic Mashed Potatoes & Rich Pan Gravy 🥔",
        "Toasted Pecan & Sourdough Sage Stuffing 🍞",
        "Brown-Butter Sweet Potato Casserole with Toasted Marshmallows 🍠",
        "Classic Spiced Pumpkin Pie with Whipped Cream 🥧",
        "Green Bean Almondine with Crispy Shallots 🫘",
        "Warm Apple Cider with Cinnamon Sticks 🍎",
    ];
    DISHES[(day as usize) % DISHES.len()]
}

/// Returns ASCII art of a festive Thanksgiving turkey.
pub fn ascii_turkey(greeting: &str) -> String {
    format!(
        r#"   .--.
  / ,. \   🍂  ______________________________
 | (  ) |     /                              \
  \ `' /  .__|  {greeting:<28} |
   `--' .`    \______________________________/
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

/// Formats a greeting message for the given target name.
pub fn greet(name: &str) -> String {
    let trimmed = name.trim();
    if trimmed.is_empty() {
        "Hello, world!".to_string()
    } else {
        format!("Hello, {trimmed}!")
    }
}

/// Generates a seasonal November & Thanksgiving status block if the date is in November
/// (or when explicitly requested via `--feast`).
pub fn seasonal_message(date: SimpleDate, force_feast: bool) -> Option<String> {
    if date.month != 11 && !force_feast {
        return None;
    }

    let tday = thanksgiving_day_of_november(date.year);
    let mut lines = Vec::new();

    if date.month == 11 {
        lines.push(format!(
            "🍁 Happy November ({:04}-{:02}-{:02})! Cozy autumn vibes detected. 🍂",
            date.year, date.month, date.day
        ));
        if date.day < tday {
            let days_left = tday - date.day;
            let day_word = if days_left == 1 { "day" } else { "days" };
            lines.push(format!(
                "🦃 Countdown: {days_left} {day_word} until Thanksgiving (Nov {tday}, {})!",
                date.year
            ));
        } else if date.day == tday {
            lines.push(
                "🦃🥧 HAPPY THANKSGIVING! Pass the gravy and enjoy the feast! 🥧🦃".to_string(),
            );
        } else {
            let days_since = date.day - tday;
            lines.push(format!(
                "🥪 Thanksgiving was {days_since} day(s) ago — prime time for turkey leftover sandwiches!"
            ));
        }
    } else {
        lines.push(format!(
            "🍂 Off-season Thanksgiving preview! In {}, Thanksgiving falls on Nov {tday}.",
            date.year
        ));
    }

    lines.push(format!(
        "🍽️  Featured Feast Dish: {}",
        dish_of_the_day(date.day)
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
    println!("Prints a friendly greeting with special November & Thanksgiving festivities!");
    println!();
    println!("Options:");
    println!("  -h, --help              Show this help message");
    println!("  -t, --turkey            Display an ASCII art Thanksgiving turkey");
    println!("  -f, --feast             Show the Thanksgiving countdown & featured dish (even outside November)");
    println!("      --date YYYY-MM-DD   Override the date (e.g. --date 2026-11-26)");
}

fn main() {
    let mut args = env::args();
    let program = args.next().unwrap_or_else(|| "hello".to_string());

    let mut show_turkey = false;
    let mut force_feast = false;
    let mut custom_date: Option<SimpleDate> = None;
    let mut name_parts: Vec<String> = Vec::new();

    while let Some(arg) = args.next() {
        match arg.as_str() {
            "-h" | "--help" => {
                print_usage(&program);
                return;
            }
            "-t" | "--turkey" => show_turkey = true,
            "-f" | "--feast" => force_feast = true,
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

    let greeting = greet(&name);
    let today = custom_date.unwrap_or_else(current_date);

    // Show the ASCII turkey if requested OR automatically during November!
    if show_turkey || today.month == 11 {
        println!("{}", ascii_turkey(&greeting));
    } else {
        println!("{greeting}");
    }

    if let Some(seasonal) = seasonal_message(today, force_feast) {
        println!("\n{seasonal}");
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn greets_default_world() {
        assert_eq!(greet("world"), "Hello, world!");
    }

    #[test]
    fn greets_custom_name() {
        assert_eq!(greet("Rust"), "Hello, Rust!");
    }

    #[test]
    fn falls_back_on_empty_input() {
        assert_eq!(greet("   "), "Hello, world!");
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
}


