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
    let mut result: Vec<&'static str> = Vec::new();
    let mut student_index: usize = 0;
    // split the string diagram by \n
    if let Some((first, second)) = diagram.split_once("\n") {
        // iterate through each simultaneously using zip
        for (index, (front, back)) in first.chars().zip(second.chars()).enumerate() {
            if index.rem_euclid(2) == 0 {
                student_index += 1;
            };
            // match if the student is correct, counting by twos
            if students[student_index - 1] == student {
                // match the pattern to plants
                if let Some(plant_type) = plant_types.get(&front) {
                    result.push(plant_type);
                }

                if let Some(plant_type) = plant_types.get(&back) {
                    result.push(plant_type);
                }
            }
        }
        result.swap(1, 2);
        result
    } else {
        panic!("Diagram incomplete - no newline found!");
    }
}
