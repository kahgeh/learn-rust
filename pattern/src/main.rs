fn pattern_used_as_loop_check(){
    let mut stack = vec![1, 2, 3, 4, 5];

    while let Some(val) = stack.pop() {
        println!("{}", val)
    }
}

fn multiple_patterns (x: i32){
    match x {
        1|2 => println!("one or two"),
        3 => println!("three"),
        4..=10 => println!("four to ten"),
        _ => println!("everything else")
    }
}

struct Point {
    x : u32,
    y: u32,
}

fn destructure_patterns (pt:&Point){
    match *pt {
        Point { x, y:0 } => println!("along horizontal line"),
        Point { x:0, y } => println!("along vertical line"),
        Point { x, y } => println!("neither along horizontal or vertical line"),
    }
}

fn pattern_with_bindings (){
    enum Message {
        Hello { id:i32 },
    }

    let msg = Message::Hello { id: 5 };

    match msg {
        Message::Hello {
            id: id_variable @ 3..=7
        } => println!("Found id in range {}", id_variable),
        Message::Hello { id: 10..=12 } => println!("found id in another range"),
        Message::Hello { id } => println!("Found some other id:{}", id),
    }
}
fn main() {
    pattern_used_as_loop_check();
    multiple_patterns(7);
    destructure_patterns(&Point { x:0, y:10 });
    pattern_with_bindings();
}
