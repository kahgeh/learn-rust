mod draw_trait;
mod console;

use crate::console::{Triangle, Text};
use crate::draw_trait::{Screen};

fn main() {
    let screen = Screen {
        components: vec![
            Box::new(Text::new("hello")),
            Box::new(Triangle::new())
        ]
    };
    screen.run();
}
