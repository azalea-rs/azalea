pub fn to_camel_case(snake_case: &str) -> String {
    let mut camel_case = String::new();
    let mut capitalize_next = true;
    for c in snake_case.chars() {
        if c == '_' {
            capitalize_next = true;
        } else {
            if capitalize_next {
                camel_case.push(c.to_ascii_uppercase());
            } else {
                camel_case.push(c);
            }
            capitalize_next = false;
        }
    }
    camel_case
}
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
