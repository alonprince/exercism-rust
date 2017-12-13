pub fn reverse(input: &str) -> String {
    let mut result = String::new();
    
    for string in input.chars().rev() {
        result = result + &string.to_string();
    }
    result
}
