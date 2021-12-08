pub struct TranslatableComponent {
    pub key: String,
    pub args: Vec<String>,
}

impl TranslatableComponent {
    pub fn new(key: String, args: Vec<String>) -> Self {
        Self { key, args }
    }
}
