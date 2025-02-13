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


