fn main() {
    //simple_ownership_example()
    move_ownership();
}

fn take_ownership(s: String) {
    println!("take_ownership {}", s);
}

fn take_and_give_back_ownership(s: String) -> String {
    s
}

fn calculate_length(s: &String) -> usize {
    s.len()
}

fn append(s: &mut String) {
    s.push_str("another");
}

// fn simple_ownership_example(){
//     let n1 = 1;
//     let n2 = n1;
//
//     println!("n1={}, n2={}", n1, n2);
//
//     // doesn't work
//     // let s1 = String::from("string 1");
//     // let s2 = s1;
//
//     // println!("s1={}, s2={}", s1, s2);
//
//     let s3 = String::from("string 3");
//     let s4 = s3.clone();
//
//     println!("s3='{}', s4='{}'", s3, s4);
//
//     let s5 = String::from("string 5");
//     take_ownership(s5);
//     // cannot compile because s5 ownership is moved to the function
//     // println!("main {}", s5);
//
//     let s6 = String::from("string 6");
//     println!("{}", take_and_give_back_ownership(s6));
//
//     let mut s7 = String::from("string 7");
//     println!("{} {}", calculate_length(&s7), calculate_length(&s7));
//     let _s9 = &s7;
//     let _s10 = &mut s7;
//     // append(&mut s7);
//     //append(_s8);
//     println!("{} {} {}", s7, _s9, _s10);
// }

struct SomeType<'a> {
    prop1: &'a str,
}

fn move_ownership(){
    let mut s = SomeType {
        prop1: "test"
    };

    let prop1 = s.prop1;

    s.prop1 = "another reassign";

    println!("{} {}", prop1, s.prop1);
}
