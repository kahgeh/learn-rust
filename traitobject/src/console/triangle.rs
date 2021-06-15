use crate::draw_trait::Draw;

pub struct Triangle {
    triangle: String,
}

impl Triangle {
    pub fn new() -> Triangle {
        Triangle {
            triangle: String::from("/_\\"),
        }
    }
}

impl Draw for Triangle {
    fn draw(&self) {
        println!("{}", self.triangle);
    }
}
