//Function to convert seconds into 24 hour time formart.
mod time;
use std::io;

fn main() {
    let mut input = String::new();

    println!("Enter the time of the day in seconds(0 to 86,399):");
    io::stdin()
        .read_line(&mut input)
        .expect("Failed to read line");

    let total_seconds: u32 = input
        .trim()
        .parse()
        .expect("Input number only without any sign!");

    if total_seconds > 86399 {
        panic!("Your input should be between 0 to 86,399 ");
    }

    let (hours, minutes, seconds) = time::convert_seconds(total_seconds);
    println!("Time: {:02}:{:02}:{:02}", hours, minutes, seconds);
}
