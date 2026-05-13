pub fn build_proverb(list: &[&str]) -> String {
    if list.is_empty() {
        return String::new();
    }

    let mut output = Vec::new();
    for i in 0..list.len() - 1 {
        let input = format!("For want of a {} the {} was lost.", list[i], list[i + 1]);
        output.push(input);
    }

    let input = format!("And all for the want of a {}.", list[0]);
    output.push(input);

    output.join("\n")
}
