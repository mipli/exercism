pub fn reply(message: &str) -> &str {
    let msg = message.trim();
    if msg.is_empty() {
        return "Fine. Be that way!";
    }
    let is_question = msg.ends_with('?');
    let has_letters = msg.contains(char::is_alphabetic);
    let is_upper = has_letters && msg == msg.to_uppercase();

    match (is_upper, is_question) {
        (true, true) => "Calm down, I know what I'm doing!",
        (true, _) => "Whoa, chill out!",
        (_, true) => "Sure.",
        _ => "Whatever."
    }
}
