use arboard::Clipboard;
use clap::Parser;
use rand::distributions::{Alphanumeric, Distribution, Uniform};
use rand::{thread_rng, Rng};
use std::io::{self, Write};
use std::thread::sleep;
use std::time::Duration;

/// Generate a random password
#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    /// Ask for password length and symbol preference
    #[arg(long)]
    ask: bool,
}

fn generate_password(length: usize, include_symbols: bool) -> String {
    let mut rng = thread_rng();

    let symbols: Vec<char> = "!@#$%^&*()-=_+[]{}|;:',.<>?/".chars().collect();

    let dist = if include_symbols {
        Uniform::new(0, 2) // 0 for alphanumeric, 1 for symbol
    } else {
        Uniform::new(0, 1)
    };

    (0..length)
        .map(|_| {
            if dist.sample(&mut rng) == 0 {
                rng.sample(Alphanumeric) as char
            } else {
                let idx = rng.gen_range(0..symbols.len());
                symbols[idx]
            }
        })
        .collect()
}

fn destroy_password(password: String) -> usize {
    let mut bytes = password.into_bytes(); // Convert String to bytes.
    let mut rng = thread_rng();
    rng.fill(&mut bytes[0..]); // Overwrite bytes with garbage.
    bytes.len()
}

fn main() {
    let args = Args::parse();

    let (length, include_symbols) = if args.ask {
        let mut length_str = String::new();
        let length: usize; // Declare length outside the loop
        loop {
            println!("Enter desired password length (minimum 10, maximum 100):");
            io::stdin()
                .read_line(&mut length_str)
                .expect("Failed to read line");
            length = match length_str.trim().parse() {
                Ok(num) if (10..=100).contains(&num) => num,
                _ => {
                    println!("Invalid input. Please enter a number between 10 and 100.");
                    length_str.clear();
                    continue;
                }
            };
            break; // Exit the loop when a valid length is entered
        }

        let mut include_symbols_str = String::new();
        println!("Include symbols? (yes/no):");
        io::stdin()
            .read_line(&mut include_symbols_str)
            .expect("Failed to read line");
        let include_symbols: bool = include_symbols_str.trim().to_lowercase() == "yes";

        (length, include_symbols)
    } else {
        (50, true) // Default values
    };

    // Display the password
    let password = generate_password(length, include_symbols);

    println!(
        "Generated password: {}{}{}",
        &password[0..5],
        "●".repeat(length - 7),
        &password[length - 3..]
    );

    // Copy to clipboard
    let mut clipboard = Clipboard::new().unwrap();
    clipboard.set_text(password.clone()).unwrap();

    let len = destroy_password(password);

    // #[cfg(debug_assertions)] // Only check in debug builds.
    assert_eq!(length, len);

    // Display the timer
    for i in (1..=15).rev() {
        print!("\rSeconds remaining: {:2}", i);
        io::stdout().flush().unwrap();
        sleep(Duration::from_secs(1));
    }

    // Clear the clipboard
    clipboard.clear().unwrap();

    // Move up one line
    print!("\x1b[1A");
    // Overwrite the password and timer with backspaces
    print!("\r{}", " ".repeat(len + 100)); // Clear the password line
    print!("\r{}", " ".repeat(50)); // Clear the timer line
    print!("\r"); // Move the cursor to the beginning of the line
    io::stdout().flush().unwrap();

    println!("Password has been hidden and removed from clipboard.");
}
