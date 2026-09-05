use crate::student::Student;
use std::io::{self, Write};

pub fn read_input(prompt: &str) -> String {
    print!("{prompt}");
    io::stdout().flush().expect("Could not flush output.");

    let mut input = String::new();
    io::stdin()
        .read_line(&mut input)
        .expect("Could not read input.");

    input.trim().to_string()
}

pub fn read_u32(prompt: &str) -> u32 {
    loop {
        let input = read_input(prompt);

        match input.parse::<u32>() {
            Ok(value) => return value,
            Err(_) => println!("Please enter a valid whole number."),
        }
    }
}

pub fn read_marks(prompt: &str) -> f32 {
    loop {
        let input = read_input(prompt);

        match input.parse::<f32>() {
            Ok(value) if (0.0..=100.0).contains(&value) => return value,
            _ => println!("Please enter marks from 0 to 100."),
        }
    }
}

pub fn show_menu() {
    println!();
    println!("================================");
    println!("      STUDENT RECORD SYSTEM");
    println!("================================");
    println!("1. Add Student");
    println!("2. Display Students");
    println!("3. Search Student");
    println!("4. Update Student");
    println!("5. Delete Student");
    println!("6. Search by Department");
    println!("7. Save Records");
    println!("8. Load Records");
    println!("9. Exit");
    println!("================================");
}

pub fn display_student(student: &Student) {
    println!("-------------");
    println!("ID: {}", student.id);
    println!("Name: {}", student.name);
    println!("Department: {}", student.department);
    println!("Semester: {}", student.semester);
    println!("Marks: {}", student.marks);
    println!("Grade: {}", student.grade());
}

pub fn pause() {
    let _ = read_input("\nPress Enter to continue...");
}
