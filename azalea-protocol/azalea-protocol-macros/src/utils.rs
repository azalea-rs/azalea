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
pub fn to_snake_case(camel_case: &str) -> String {
    let mut snake_case = String::new();
    for c in camel_case.chars() {
        if c.is_ascii_uppercase() {
            snake_case.push('_');
            snake_case.push(c.to_ascii_lowercase());
        } else {
            snake_case.push(c);
        }
    }
    snake_case
}
