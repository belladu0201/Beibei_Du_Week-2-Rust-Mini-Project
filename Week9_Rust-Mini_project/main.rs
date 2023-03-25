use std::time::Duration;
use chrono::prelude::*;
use std::thread;

// start the timer
fn start_timer() -> std::time::Duration {
    let start = Local::now();
    thread::sleep(Duration::from_secs(5));
    let end = Local::now();
    let elapsed = end - start;
    elapsed.to_std().unwrap()
}

// convert the time elapsed to days
fn convert_to_days(elapsed: Duration) -> f64 {
    let days = elapsed.as_secs_f64() / 86400.0;
    days
}

// calculate the number of days until a target date
fn days_until(target: NaiveDateTime) -> f64 {
    let now = Local::now().naive_local();
    let duration = target.signed_duration_since(now);
    let days = duration.num_seconds() as f64 / 86400.0;
    days
}

// main function
fn main() {
    let elapsed = start_timer();
    let days = convert_to_days(elapsed);
    println!("{} days", days);

    let target_date = NaiveDate::from_ymd_opt(2023, 4, 19).expect("Invalid date").and_hms_opt(0, 0, 0).expect("Invalid time");
    let days = days_until(target_date);
    println!("{} days until April 19, 2023, the last class in SPRING 2023 for Duke Graduate student", days);
}
