use crate::Header;

#[derive(Debug, Clone, Copy)]
pub struct ContentLength(i32);

impl ContentLength {
    pub fn new(count: i32) -> Self {
        Self(count)
    }
}

impl Header for ContentLength {
    fn name(&self) -> &'static str {
        http::header::CONTENT_LENGTH.as_str()
    }

    fn value(&self) -> String {
        let count = if self.0 < 0 { -1 } else { self.0 };
        format!("{}", count)
    }
}
