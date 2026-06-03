const STUDENTS: [&str; 12] = [
    "Alice", "Bob", "Charlie", "David", "Eve", "Fred", "Ginny", "Harriet", "Ileana", "Joseph",
    "Kincaid", "Larry",
];

pub fn plants(diagram: &str, student: &str) -> Vec<&'static str> {
    let idx = STUDENTS.iter().position(|&s| s == student).unwrap() * 2;

    diagram
        .lines()
        .flat_map(|line| {
            line[idx..idx + 2].chars().map(|plant| match plant {
                'C' => "clover",
                'G' => "grass",
                'R' => "radishes",
                'V' => "violets",
                _ => "",
            })
        })
        .collect()
}
