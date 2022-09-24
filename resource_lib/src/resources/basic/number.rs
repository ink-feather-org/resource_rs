use std::fmt::Display;

#[derive(Debug)]
pub struct Number {
    content: i32,
}
impl Number {
    pub fn new(content: i32) -> Self {
        Self { content }
    }
}

impl Display for Number {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.content)
    }
}
