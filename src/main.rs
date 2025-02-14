mod countdown_timer;

use std::io;
use std::io::{stdout, Write};

use crate::countdown_timer::{CountdownTimer, TimeUnit};

#[tokio::main]
async fn main() {
    println!("Please enter countdown start in seconds:");
    stdout().flush().unwrap();
    let mut start_input = String::new();
    io::stdin().read_line(&mut start_input).expect("Failed to read start");

    let start: u64 = match start_input.trim().parse() {
        Ok(num) => num,
        Err(_) => {
            println!("Please enter a valid integer.");
            return;
        }
    };


    let mut timer = CountdownTimer::new(start, TimeUnit::SECOND);
    println!();
    println!("Countdown Timer is starting....");
    timer.start().await;
    println!("\n💣💣💣💣");
}

#[cfg(test)]
mod tests {

    #[test]
    fn test_valid_start_input() {
        let start_input = "1".to_string();

        let start: u64 = match start_input.trim().parse() {
            Ok(num) => num,
            Err(_) => {
                println!("Please enter a valid integer.");
                return;
            }
        };

        assert_eq!(start, 1);
    }

    #[test]
    #[should_panic]
    fn test_invalid_start_input() {
        let start_input = "a".to_string();

        let start: u64 = match start_input.trim().parse() {
            Ok(num) => num,
            Err(_) => {
                panic!("Please enter a valid integer");
            }
        };

    }
}
