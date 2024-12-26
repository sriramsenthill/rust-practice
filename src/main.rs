// main.rs
mod ownership_and_borrowing;
mod structs;
mod time;
use std::io;

fn main() {
    match get_user_input() {
        Ok(seconds) => {
            let (hours, minutes, seconds) = time::convert_seconds(seconds);
            println!("Time: {:02}:{:02}:{:02}", hours, minutes, seconds);
        }
        Err(e) => {
            eprintln!("Error: {}", e);
            std::process::exit(1);
        }
    }

    ownership_and_borrowing::ownership_and_borrowing();

    // Using the constructor
    let person1 = structs::User::new(
        String::from("Bob"),
        25,
        String::from("bob123"),
        String::from("bob@example.com"),
    );

    // Using the default person creator
    let person2 = structs::User::create_default_person();

    // Printing details
    println!("Person 1:");
    person1.print_details();

    println!("\nPerson 2:");
    person2.print_details();

    // You can also print the struct directly using Debug trait
    println!("\nPerson 1 debug print: {:?}", person1);
}

pub fn get_user_input() -> Result<u32, String> {
    println!("Enter the time of the day in seconds(0 to 86,399):");
    let mut input = String::new();

    io::stdin()
        .read_line(&mut input)
        .map_err(|_| "Failed to read input".to_string())?;

    parse_input(&input)
}

// Separated the parsing logic for easier testing
pub fn parse_input(input: &str) -> Result<u32, String> {
    let trimmed = input.trim();
    if trimmed.is_empty() {
        return Err("Input cannot be empty".to_string());
    }

    match trimmed.parse::<i32>() {
        Ok(num) if num < 0 => Err("Input cannot be negative".to_string()),
        Ok(num) if num > 86399 => Err("Input must be between 0 and 86,399".to_string()),
        Ok(num) => Ok(num as u32),
        Err(_) => Err("Invalid input: please enter a valid number".to_string()),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_valid_input() {
        let result = parse_input("3600\n");
        assert!(result.is_ok());
        assert_eq!(result.unwrap(), 3600);
    }

    #[test]
    fn test_zero_input() {
        let result = parse_input("0\n");
        assert!(result.is_ok());
        assert_eq!(result.unwrap(), 0);
    }

    #[test]
    fn test_max_valid_input() {
        let result = parse_input("86399\n");
        assert!(result.is_ok());
        assert_eq!(result.unwrap(), 86399);
    }

    #[test]
    fn test_negative_input() {
        let result = parse_input("-1\n");
        assert!(result.is_err());
        assert_eq!(result.unwrap_err(), "Input cannot be negative");
    }

    #[test]
    fn test_too_large_input() {
        let result = parse_input("86400\n");
        assert!(result.is_err());
        assert_eq!(result.unwrap_err(), "Input must be between 0 and 86,399");
    }

    #[test]
    fn test_empty_input() {
        let result = parse_input("\n");
        assert!(result.is_err());
        assert_eq!(result.unwrap_err(), "Input cannot be empty");
    }

    #[test]
    fn test_alphabetic_input() {
        let result = parse_input("abc\n");
        assert!(result.is_err());
        assert_eq!(
            result.unwrap_err(),
            "Invalid input: please enter a valid number"
        );
    }

    #[test]
    fn test_special_characters() {
        let result = parse_input("@#$\n");
        assert!(result.is_err());
        assert_eq!(
            result.unwrap_err(),
            "Invalid input: please enter a valid number"
        );
    }

    #[test]
    fn test_floating_point() {
        let result = parse_input("3600.5\n");
        assert!(result.is_err());
        assert_eq!(
            result.unwrap_err(),
            "Invalid input: please enter a valid number"
        );
    }

    #[test]
    fn test_whitespace() {
        let result = parse_input("  3600  \n");
        assert!(result.is_ok());
        assert_eq!(result.unwrap(), 3600);
    }
}
