use std::env;

/// Formats a greeting message for the given target name.
pub fn greet(name: &str) -> String {
    let trimmed = name.trim();
    if trimmed.is_empty() {
        "Hello, world!".to_string()
    } else {
        format!("Hello, {trimmed}!")
    }
}

fn print_usage(program: &str) {
    println!("Usage: {program} [NAME...]");
    println!();
    println!("Prints a friendly greeting. Defaults to \"Hello, world!\" when no name is provided.");
}

fn main() {
    let mut args = env::args();
    let program = args.next().unwrap_or_else(|| "hello".to_string());
    let rest: Vec<String> = args.collect();

    if rest.iter().any(|arg| arg == "-h" || arg == "--help") {
        print_usage(&program);
        return;
    }

    let name = if rest.is_empty() {
        "world".to_string()
    } else {
        rest.join(" ")
    };

    println!("{}", greet(&name));
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
}

