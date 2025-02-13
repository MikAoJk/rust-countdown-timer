use std::io::{stdout, Write};
use std::time::Duration;
use tokio::time;


pub struct CountdownTimer {
    start: u64,
    time_unit: TimeUnit,
}

impl CountdownTimer {
    pub fn new(start: u64, time_unit: TimeUnit) -> Self {
        CountdownTimer {
            start,
            time_unit,
        }
    }

    pub(crate) async fn start(&mut self) {
        let mut task_interval = time::interval(Duration::from_secs(1));
        let seconds = self.to_seconds();

        for i in (0..=seconds).rev() {
            task_interval.tick().await;
            print!("\r{}    ", i);
            stdout().flush().unwrap();
        }
    }

    fn to_seconds(&self) -> u64 {
        match self.time_unit {
            // TimeUnit::HOUR => self.start * 60 * 60,
            // TimeUnit::MINUTE => self.start * 60,
            TimeUnit::SECOND => self.start,
        }
    }
}

pub enum TimeUnit {
    //HOUR,
    //MINUTE,
    SECOND,
}