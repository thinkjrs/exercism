use std::collections::HashMap;

pub fn plants(diagram: &str, student: &str) -> Vec<&'static str> {
    let plant_types = HashMap::from([
        ('G', "grass"),
        ('C', "clover"),
        ('R', "radishes"),
        ('V', "violets"),
    ]);

    let students = [
        "Alice", "Bob", "Charlie", "David", "Eve", "Fred", "Ginny", "Harriet", "Ileana", "Joseph",
        "Kincaid", "Larry",
    ];

    let rows: Vec<&str> = diagram.split('\n').collect();

    if rows.len() % 2 != 0 {
        panic!("Diagram incomplete - no newline found!");
    }

    // get index of student
    let student_index = students
        .iter()
        .position(|s| *s == student)
        .expect("Student not found!");

    let result: Vec<&'static str> = rows
        .iter()
        .flat_map(|row| {
            row.chars()
                .skip(student_index * 2)
                .take(2)
                .map(|plant| *plant_types.get(&plant).expect("Invalid plant key!"))
        })
        .collect();

    result
}
