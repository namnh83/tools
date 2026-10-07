use rand::Rng;
use std::io::{self, Write};

fn main() {
    loop {
        println!("=== Password Generator ===\n");

        // 1. Password length
        let length = loop {
            print!("Password length (minimum 4): ");
            io::stdout().flush().unwrap();

            let mut input = String::new();
            io::stdin().read_line(&mut input).expect("Failed to read input");

            match input.trim().parse::<usize>() {
                Ok(n) if n >= 4 => break n,
                _ => println!("Please enter a number ≥ 4!\n"),
            }
        };

        // 2. How many passwords
        let count = loop {
            print!("How many passwords? (1-100, default 10): ");
            io::stdout().flush().unwrap();

            let mut input = String::new();
            io::stdin().read_line(&mut input).expect("Failed to read input");

            let trimmed = input.trim();
            if trimmed.is_empty() {
                break 10; // default
            }

            match trimmed.parse::<usize>() {
                Ok(n) if n >= 1 && n <= 100 => break n,
                _ => println!("Please enter a number between 1 and 100!\n"),
            }
        };

        // 3. Character types
        println!("\nWhich character types should be used?");
        println!("(Multiple selection – enter numbers separated by spaces, e.g. 1 2 4)\n");
        println!("1 = Lowercase letters (a-z)");
        println!("2 = Uppercase letters (A-Z)");
        println!("3 = Numbers (0-9)");
        println!("4 = Special characters (!@#$%^&* etc.)");

        let charset = loop {
            print!("\nSelection: ");
            io::stdout().flush().unwrap();

            let mut input = String::new();
            io::stdin().read_line(&mut input).expect("Failed to read input");

            let mut chars = String::new();
            let mut valid = false;

            for part in input.split_whitespace() {
                match part {
                    "1" => {
                        chars.push_str("abcdefghijklmnopqrstuvwxyz");
                        valid = true;
                    }
                    "2" => {
                        chars.push_str("ABCDEFGHIJKLMNOPQRSTUVWXYZ");
                        valid = true;
                    }
                    "3" => {
                        chars.push_str("0123456789");
                        valid = true;
                    }
                    "4" => {
                        chars.push_str("!@#$%^&*()_+-=[]{}|;:,.<>?");
                        valid = true;
                    }
                    _ => {}
                }
            }

            if valid && !chars.is_empty() {
                break chars;
            } else {
                println!("Please select at least one valid option (1–4)!");
            }
        };

        // 4. Generate passwords
        println!("\nGenerated passwords:");
        println!("────────────────────────────────────");

        for i in 1..=count {
            let password = generate_password(length, &charset);
            println!("{:>2}. {}", i, password);
        }

        println!("────────────────────────────────────");

        // Generate again?
        print!("\nGenerate again? (y/n): ");
        io::stdout().flush().unwrap();

        let mut again = String::new();
        io::stdin().read_line(&mut again).unwrap();

        if again.trim().to_lowercase() != "y" {
            println!("\nPress Enter to exit...");
            let mut _exit = String::new();
            io::stdin().read_line(&mut _exit).unwrap();
            break;
        }

        println!("\n");
    }
}

fn generate_password(length: usize, charset: &str) -> String {
    let mut rng = rand::thread_rng();
    let chars: Vec<char> = charset.chars().collect();

    (0..length)
        .map(|_| chars[rng.gen_range(0..chars.len())])
        .collect()
}