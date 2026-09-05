use crate::student::Student;
use std::fs;
use std::io;

const FILE_NAME: &str = "students.txt";

pub fn save_students(students: &[Student]) -> io::Result<()> {
    let content = students
        .iter()
        .map(Student::to_file_line)
        .collect::<Vec<String>>()
        .join("\n");

    fs::write(FILE_NAME, content)
}

pub fn load_students() -> io::Result<Vec<Student>> {
    match fs::read_to_string(FILE_NAME) {
        Ok(content) => {
            let mut students = Vec::new();

            for line in content.lines().filter(|line| !line.trim().is_empty()) {
                if let Ok(student) = Student::from_file_line(line) {
                    students.push(student);
                }
            }

            Ok(students)
        }
        Err(error) if error.kind() == io::ErrorKind::NotFound => Ok(Vec::new()),
        Err(error) => Err(error),
    }
}
