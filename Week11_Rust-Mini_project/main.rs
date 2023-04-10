use std::io;

fn main() {
    // ask for student's name
    let mut name = String::new();
    println!("Please enter the student's name:");
    io::stdin().read_line(&mut name).expect("Failed to read line");

    // ask for grades in different categories
    let mut homework = String::new();
    println!("Please enter the student's grade for homework (out of 100):");
    io::stdin().read_line(&mut homework).expect("Failed to read line");
    let homework: f32 = homework.trim().parse().expect("Please enter a valid number");

    let mut participation = String::new();
    println!("Please enter the student's grade for participation (out of 100):");
    io::stdin().read_line(&mut participation).expect("Failed to read line");
    let participation: f32 = participation.trim().parse().expect("Please enter a valid number");

    let mut midterm = String::new();
    println!("Please enter the student's grade for midterm (out of 100):");
    io::stdin().read_line(&mut midterm).expect("Failed to read line");
    let midterm: f32 = midterm.trim().parse().expect("Please enter a valid number");

    let mut final_proj = String::new();
    println!("Please enter the student's grade for final project (out of 100):");
    io::stdin().read_line(&mut final_proj).expect("Failed to read line");
    let final_proj: f32 = final_proj.trim().parse().expect("Please enter a valid number");

    let mut reading_reflection = String::new();
    println!("Please enter the student's grade for reading reflection (out of 100):");
    io::stdin().read_line(&mut reading_reflection).expect("Failed to read line");
    let reading_reflection: f32 = reading_reflection.trim().parse().expect("Please enter a valid number");

    // calculate the final grade
    let final_grade = 0.2 * homework + 0.2 * participation + 0.2 * midterm + 0.2 * final_proj + 0.2 * reading_reflection;

    // print out the final grade
    println!("{}'s final grade is {:.2}", name.trim(), final_grade);

    // calculate and print the letter grade
    let letter_grade = match final_grade {
        90.0..=100.0 => "A",
        80.0..=89.0 => "B",
        70.0..=79.0 => "C",
        60.0..=69.0 => "D",
        0.0..=59.0 => "F",
        _ => "Invalid grade",
    };
    println!("{}'s letter grade is {}", name.trim(), letter_grade);
}