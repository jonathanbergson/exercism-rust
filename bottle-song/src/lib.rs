pub fn recite(start_bottles: u32, take_down: u32) -> String {
    let mut song = String::new();

    for i in 0..take_down {
        let bottle_count = start_bottles - i;
        let bottle_count_rest = bottle_count - 1;

        let first_verse = format!("{} green {} hanging on the wall,\n", get_number_label(bottle_count), get_bottle_label(bottle_count));
        let second_verse = format!("And if one green bottle should accidentally fall,\n");
        let third_verse = format!("There'll be {} green {} hanging on the wall.\n", get_number_label(bottle_count_rest).to_lowercase(), get_bottle_label(bottle_count_rest));

        song.push_str(&first_verse);
        song.push_str(&first_verse);
        song.push_str(&second_verse);
        song.push_str(&third_verse);
        song.push_str("\n");
    }

    song
}

fn get_number_label(number: u32) -> String {
    match number {
        10 => "Ten",
        9 => "Nine",
        8 => "Eight",
        7 => "Seven",
        6 => "Six",
        5 => "Five",
        4 => "Four",
        3 => "Three",
        2 => "Two",
        1=> "One",
        0 => "No",
        _ => "?"
    }.to_string()
}

fn get_bottle_label(number: u32) -> String {
    match number {
        1 => "bottle",
        _ => "bottles",
    }.to_string()
}
