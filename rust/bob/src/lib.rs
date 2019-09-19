pub fn reply(message: &str) -> &str {
    if message.is_empty() || message.chars().all(|c| c.is_whitespace()) {
        "Fine. Be that way!"
    } else {
        let letters: Vec<char> = message.chars().filter(|c| c.is_alphabetic()).collect();
        let is_yelling = if letters.is_empty() {
            false
        } else {
            letters.iter().all(|letter| letter.is_uppercase())
        };

        let is_a_question = message.trim_end().ends_with('?');
        match (is_yelling, is_a_question) {
            (true, true) => "Calm down, I know what I'm doing!",
            (true, false) => "Whoa, chill out!",
            (false, true) => "Sure.",
            (false, false) => "Whatever.",
        }
    }
}
