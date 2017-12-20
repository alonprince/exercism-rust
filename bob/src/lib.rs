
pub fn reply(message: &str) -> &str {
    match message {
        ""                          => "Fine. Be that way!",
        _ if is_yell(message)       => "Whoa, chill out!",
        _ if is_question(message)   => "Sure.",
        _ if is_no_words(message)   => "Fine. Be that way!",
        _                           => "Whatever."
    }
}

fn is_yell(message: &str) -> bool {
    message.to_uppercase() == message &&
    has_letter(message) &&
    !is_no_words(message)
}

fn has_letter(message: &str) -> bool {
    for (_i, &item) in message.as_bytes().iter().enumerate() {
        if item > 64 && item < 123 {
            return true;
        }
    }
    false
}

fn is_question(message: &str) -> bool {
    let trim_str = message.trim();
    let len = trim_str.len();
    if len > 0 {
        if &trim_str[(len - 1)..] == "?" {
            return true;
        }
    }
    false
}

fn is_no_words(message: &str) -> bool {
    message.trim() == ""
}
