pub fn to_pascal_case(s: &str) -> String {
    // we get the first item later so this is to make it impossible for that
    // to error
    if s.is_empty() {
        return String::new();
    }

    let mut result = String::new();
    let mut prev_was_underscore = true; // set to true by default so the first character is capitalized
    if s.chars().next().unwrap().is_numeric() {
        result.push('_');
    }
    for c in s.chars() {
        if c == '_' {
            prev_was_underscore = true;
        } else if prev_was_underscore {
            result.push(c.to_ascii_uppercase());
            prev_was_underscore = false;
        } else {
            result.push(c);
        }
    }
    result
}
