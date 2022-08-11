use std::fmt::Display;

#[derive(Debug)]
pub struct Text {
    content: String,
}

impl Text {
    pub fn new(content: String) -> Self {
        Self { content }
    }
}

impl Display for Text {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.content)
    }
}
