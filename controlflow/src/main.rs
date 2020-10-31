fn main() {
    let mut counter = 0;

    let result = loop {
        counter += 1;

        if counter == 10 {
            break counter * 2;
        }
    };

    println!("The result is {}", result);

    let a = [10, 20, 30, 40];
    for element in a.iter() {
        println!("The value is {}", element)
    }

    // (1..4) = 1,2,3
    // (1..=4) = 1,2,3,4
    for number in (1..=4).rev() {
        println!("{}", number);
    }
}
