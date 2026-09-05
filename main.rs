mod file_manager;
mod menu;
mod student;

use file_manager::{load_students, save_students};
use menu::{display_student, pause, read_input, read_marks, read_u32, show_menu};
use student::Student;

fn add_student(students: &mut Vec<Student>) {
    println!("\n--- Add Student ---");

    let id = read_u32("Enter Student ID: ");

    if students.iter().any(|student| student.id == id) {
        println!("A student with this ID already exists.");
        return;
    }

    let name = read_input("Enter Name: ");
    let department = read_input("Enter Department: ");
    let semester = read_u32("Enter Semester: ");
    let marks = read_marks("Enter Marks: ");

    match Student::new(id, name, department, semester, marks) {
        Ok(student) => {
            students.push(student);
            println!("Student added successfully.");
        }
        Err(error) => println!("Could not add student: {error}"),
    }
}

fn display_students(students: &[Student]) {
    println!("\n--- All Students ---");

    if students.is_empty() {
        println!("No student records found.");
        return;
    }

    for student in students {
        display_student(student);
    }
}

fn search_student(students: &[Student]) {
    println!("\n--- Search Student ---");
    let id = read_u32("Enter Student ID: ");

    match students.iter().find(|student| student.id == id) {
        Some(student) => {
            println!("Student Found");
            display_student(student);
        }
        None => println!("Student not found."),
    }
}

fn update_student(students: &mut [Student]) {
    println!("\n--- Update Student ---");
    let id = read_u32("Enter Student ID: ");

    match students.iter_mut().find(|student| student.id == id) {
        Some(student) => {
            println!("Enter the new information.");

            let name = read_input(&format!("Name [{}]: ", student.name));
            let department = read_input(&format!("Department [{}]: ", student.department));
            let semester = read_u32(&format!("Semester [{}]: ", student.semester));
            let marks = read_marks(&format!("Marks [{}]: ", student.marks));

            if !name.is_empty() {
                student.name = name;
            }
            if !department.is_empty() {
                student.department = department;
            }

            student.semester = semester;
            student.marks = marks;

            println!("Student updated successfully.");
        }
        None => println!("Student not found."),
    }
}

fn delete_student(students: &mut Vec<Student>) {
    println!("\n--- Delete Student ---");
    let id = read_u32("Enter Student ID: ");

    let old_len = students.len();
    students.retain(|student| student.id != id);

    if students.len() < old_len {
        println!("Student deleted successfully.");
    } else {
        println!("Student not found.");
    }
}

fn search_by_department(students: &[Student]) {
    println!("\n--- Search by Department ---");
    let department = read_input("Enter Department: ");

    let matches: Vec<&Student> = students
        .iter()
        .filter(|student| student.department.eq_ignore_ascii_case(&department))
        .collect();

    if matches.is_empty() {
        println!("No students found in this department.");
    } else {
        for student in matches {
            display_student(student);
        }
    }
}

fn main() {
    let mut students = match load_students() {
        Ok(students) => students,
        Err(error) => {
            println!("Could not load records: {error}");
            Vec::new()
        }
    };

    loop {
        show_menu();

        let choice = read_input("Enter your choice: ");

        match choice.as_str() {
            "1" => add_student(&mut students),
            "2" => display_students(&students),
            "3" => search_student(&students),
            "4" => update_student(&mut students),
            "5" => delete_student(&mut students),
            "6" => search_by_department(&students),
            "7" => match save_students(&students) {
                Ok(_) => println!("Records saved successfully."),
                Err(error) => println!("Could not save records: {error}"),
            },
            "8" => match load_students() {
                Ok(loaded) => {
                    students = loaded;
                    println!("Records loaded successfully.");
                }
                Err(error) => println!("Could not load records: {error}"),
            },
            "9" => {
                println!("Goodbye!");
                break;
            }
            _ => println!("Incorrect menu choice. Please choose 1-9."),
        }

        if choice != "9" {
            pause();
        }
    }
}
