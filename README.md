# Student Record Management System

A beginner-friendly command-line Student Record Management System written in Rust.

## Features

- Add a new student
- Display all students
- Search for a student by ID
- Update student information
- Delete a student
- Search students by department
- Calculate and display a grade
- Save records to a file
- Load records from a file
- Basic automated tests

## Project Structure

```text
student_record_system/
├── Cargo.toml
├── README.md
└── src/
    ├── main.rs
    ├── student.rs
    ├── menu.rs
    └── file_manager.rs
```

## Requirements

Install Rust and Cargo from the official Rust website:

https://www.rust-lang.org/tools/install

## Run the Project

Open a terminal inside the project folder and run:

```bash
cargo run
```

## Run Tests

```bash
cargo test
```

## Data File

The program saves student records in `students.txt` in the project folder when you choose **Save Records**.

## Grade Scale

The implementation uses this simple scale:

- 90-100: A+
- 80-89.99: A
- 70-79.99: B+
- 60-69.99: B
- 50-59.99: C
- 40-49.99: D
- Below 40: F

## Rust Concepts Demonstrated

This project demonstrates variables and data types, functions, structs, pattern matching, vectors, strings, ownership and borrowing, error handling, modules, file I/O, iterators, and testing.
