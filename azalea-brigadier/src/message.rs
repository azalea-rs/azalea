use std::rc::Rc;

pub struct Message(Rc<String>);

impl Message {
    pub fn string(&self) -> String {
        self.0.to_string()
    }
}

impl From<String> for Message {
    fn from(s: String) -> Self {
        Self(Rc::new(s))
    }
}
