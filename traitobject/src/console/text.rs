use crate::draw_trait::Draw;

pub struct Text {
    text: String,
}

impl Text {
    pub fn new(text: &str) -> Text {
        Text {
            text: String::from(text)
        }
    }
}

impl Draw for Text {
    fn draw(&self) {
        println!("{}", self.text);
    }
}