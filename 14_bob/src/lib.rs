pub fn reply(message: &str) -> &str {
    let is_questioning = message.trim().ends_with('?');
    let is_yelling =
        message.chars().any(|ch| ch.is_alphabetic()) && message == message.to_uppercase();
    let is_empty = message.trim().is_empty();

    match (is_questioning, is_yelling, is_empty) {
        (true, true, false) => "Calm down, I know what I'm doing!",
        (false, false, true) => "Fine. Be that way!",
        (false, true, false) => "Whoa, chill out!",
        (true, false, false) => "Sure.",
        (_, _, _) => "Whatever.",
    }
}
