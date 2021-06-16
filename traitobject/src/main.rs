mod draw;
mod console;

mod monomorphy;
use crate::monomorphy::Screen;
use crate::console::{Triangle, Text};
fn main() {
    // allow different types but only one homogeneous type at any one time
    let screen = Screen {
        components: vec![
            Triangle::new(),
            Triangle::new()
        ]
    };
    screen.run();

    let screen = Screen {
        components: vec![
            Text::new("hello"),
            Text::new("world"),
            //Triangle::new() // not allowed as screen is Screen<T> where there can only be one type of T at any one time
        ]
    };
    screen.run();
}

// mod traity;
// use crate::traity::{Screen};
// fn main() {
//     let screen = Screen {
//         components: vec![
//             Box::new(Text::new("hello")),
//             Box::new(Triangle::new())
//         ]
//     };
//     screen.run();
// }
