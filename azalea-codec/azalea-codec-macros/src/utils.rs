pub fn to_snake_case(s: &str) -> String {
    let mut result = String::new();
    let mut prev_was_uppercase = true;
    for c in s.chars() {
        if c.is_ascii_uppercase() {
            if !prev_was_uppercase {
                result.push('_');
            }
            result.push(c.to_ascii_lowercase());
            prev_was_uppercase = true;
        } else {
            result.push(c);
            prev_was_uppercase = false;
        }
    }
    result
}
