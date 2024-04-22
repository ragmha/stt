use std::{fs::File, io::{self, Write}};
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};


#[derive(Serialize, Deserialize)]
struct TimeEntry {
    start_time: DateTime<Utc>,
    end_time: DateTime<Utc>,
    duration: i64, 
}

impl TimeEntry {
fn new(start_time: DateTime<Utc>, end_time: DateTime<Utc>, duration: i64) -> Self {
        TimeEntry { start_time, end_time, duration }
    }
}


#[derive(Serialize, Deserialize)]
struct TimeTracker {
  entries: Vec<TimeEntry>
}

impl TimeTracker {
    fn new() -> TimeTracker {
        TimeTracker { entries: Vec::new() }
    }

    fn start(&mut self) {
        let start_time = Utc::now();
        println!("Timer started at: {}", start_time);

        let entry = TimeEntry::new(start_time, start_time, 0);
        self.entries.push(entry);
    }

    fn stop(&mut self) {
        if let Some(entry) = self.entries.last_mut() {
             let end_time = Utc::now();
             println!("Timer stopped at: {}", end_time);

             entry.end_time = end_time;

             let duration = entry.end_time.signed_duration_since(entry.start_time).num_seconds();
             entry.duration = duration;
             
             println!("Duration: {} seconds", duration);
        } else {
            println!("Timer is not running.")
        }
    }

    fn save_to_file(&self) -> io::Result<()> {
        let today = Utc::now();
        let filename = format!("{}.json", today.format("%Y-%m-%d"));

        let data = serde_json::to_string_pretty(&self)?;

        let mut file = File::create(filename)?;
        file.write_all(data.as_bytes())?;

        Ok(())
    }
}


fn main() {
    let mut tracker = TimeTracker::new();
    let mut should_exit = false;

    while !should_exit {
        print!("Enter command (start/stop/exit): ");
        io::stdout().flush().unwrap();

        let mut command = String::new();
        io::stdin().read_line(&mut command).expect("Failed to read line");

        match command.trim() {
            "start" => tracker.start(),
            "stop" => tracker.stop(),
            "exit" => {
                should_exit = true;
                println!("Saving time tracking data");

                if let Err(err) = tracker.save_to_file() {
                    eprintln!("Error saving data: {}", err);
                } else {
                    println!("Data saved successfully");
                }
            }
            _ => println!("Invalid command, Please enter 'start', 'stop' or 'exit' "),
        }
    }
}