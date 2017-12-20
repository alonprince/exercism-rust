enum MessageType {
    Question,
    Yell,
    NoWords,
    Other
}

pub fn reply(message: &str) -> &str {
    match analyse_message_type(message) {
        MessageType::Question => "Sure.",
        MessageType::Yell => "Whoa, chill out!",
        MessageType::NoWords => "Fine. Be that way!",
        MessageType::Other => "Whatever."
    }
}

fn analyse_message_type(message: &str) -> MessageType {
    if is_yell(message) {
        MessageType::Yell
    } else if is_question(message) {
        MessageType::Question
    } else if is_no_words(message) {
        MessageType::NoWords
    } else {
        MessageType::Other
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
