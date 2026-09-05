#[derive(Debug, Clone)]
pub struct Student {
    pub id: u32,
    pub name: String,
    pub department: String,
    pub semester: u32,
    pub marks: f32,
}

impl Student {
    pub fn new(
        id: u32,
        name: String,
        department: String,
        semester: u32,
        marks: f32,
    ) -> Result<Self, String> {
        if !(0.0..=100.0).contains(&marks) {
            return Err("Marks must be between 0 and 100.".to_string());
        }

        if semester == 0 {
            return Err("Semester must be at least 1.".to_string());
        }

        Ok(Self {
            id,
            name,
            department,
            semester,
            marks,
        })
    }

    pub fn grade(&self) -> &'static str {
        match self.marks {
            90.0..=100.0 => "A+",
            80.0..90.0 => "A",
            70.0..80.0 => "B+",
            60.0..70.0 => "B",
            50.0..60.0 => "C",
            40.0..50.0 => "D",
            _ => "F",
        }
    }

    pub fn to_file_line(&self) -> String {
        format!(
            "{}|{}|{}|{}|{}",
            self.id,
            self.name.replace('|', "/"),
            self.department.replace('|', "/"),
            self.semester,
            self.marks
        )
    }

    pub fn from_file_line(line: &str) -> Result<Self, String> {
        let parts: Vec<&str> = line.split('|').collect();

        if parts.len() != 5 {
            return Err("Invalid record format.".to_string());
        }

        let id = parts[0]
            .parse::<u32>()
            .map_err(|_| "Invalid student ID.".to_string())?;

        let semester = parts[3]
            .parse::<u32>()
            .map_err(|_| "Invalid semester.".to_string())?;

        let marks = parts[4]
            .parse::<f32>()
            .map_err(|_| "Invalid marks.".to_string())?;

        Student::new(
            id,
            parts[1].to_string(),
            parts[2].to_string(),
            semester,
            marks,
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn grade_calculation_works() {
        let student = Student::new(
            101,
            "Ali".to_string(),
            "Computer Science".to_string(),
            4,
            78.0,
        )
        .unwrap();

        assert_eq!(student.grade(), "B+");
    }

    #[test]
    fn invalid_marks_are_rejected() {
        let result = Student::new(
            101,
            "Ali".to_string(),
            "Computer Science".to_string(),
            4,
            120.0,
        );

        assert!(result.is_err());
    }

    #[test]
    fn file_conversion_works() {
        let student = Student::new(
            101,
            "Ali".to_string(),
            "Computer Science".to_string(),
            4,
            78.0,
        )
        .unwrap();

        let restored = Student::from_file_line(&student.to_file_line()).unwrap();

        assert_eq!(restored.id, 101);
        assert_eq!(restored.name, "Ali");
        assert_eq!(restored.marks, 78.0);
    }
}
