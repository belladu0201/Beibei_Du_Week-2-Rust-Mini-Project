# Week9_Rust-Mini_project: 
# Rust Countdown Timer

This is a Rust program that calculates the number of days until a target date. It uses the `chrono` library to handle dates and times.

# Motivation

I created this application because I wanted to know how many days were left until the last day of our class, that is April 18, 2023. However, I realized that it could be used for many other countdown purposes, such as counting down to a birthday, a vacation, or a project deadline. Users can set the countdown to be any date, with a countdown in seconds, minutes, hours, days, weeks, months, or years. All things can be set manually depending on the needs.

## How to Run

1. Install Rust by following the instructions at https://www.rust-lang.org/tools/install.
2. Clone this repository: `git clone https://github.com/belladu0201/Beibei_Du_Weekly-Rust-Mini-Project.git`
3. Navigate to the repository: `cd rust-countdown-timer`
4. Run the program: `cargo run`
5. The program will output the number of days until April 19, 2023, which is the date of last class for Duke University graduate students.

## How it Works

The program uses the `chrono` library to handle dates and times. It defines several functions:

- `start_timer()` starts a timer and returns the elapsed time as a `std::time::Duration`.
- `convert_to_days(elapsed: Duration) -> f64` converts the elapsed time to days.
- `days_until(target: NaiveDateTime) -> f64` calculates the number of days until a target date.

The `main()` function calls these functions to start the timer, convert the elapsed time to days, and calculate the number of days until April 19, 2023. The program outputs the result to the console.

## Output Display
<img width="904" alt="Screen Shot 2023-03-25 at 4 00 23 PM" src="https://user-images.githubusercontent.com/60382493/227738917-53dcdbc3-8512-439b-b979-7753df9de48d.png">
