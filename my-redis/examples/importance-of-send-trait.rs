use std::rc::Rc;
use tokio::task::yield_now;

#[tokio::main]
async fn main() {
    tokio::spawn(async {
        let val = String::from("bizzaro");
        {
            let rc = Rc::new("hello");
            println!("{}", rc);
        }
        yield_now().await;
        println!("{}", val);

        // Given that the async task, the continuation bit in this case can run in different thread
        // all state need to be implement the Send trait, so this will not compile because
        // Rc does not implement the Send trait
        // note 1: most included types in rust implements the Send trait, so that it's values can be
        // moved between threads - however this is not always the safe. Types that wrap around
        // underlying values so that it can be borrowed multiple times without ensuring atomicity in
        // the reference counting, e.g. Rc<T> are not safe and thus does not implement the Send trait
        /*
        let rc = Rc::new("hello");
        println!("{}", rc);
        yield_now().await;
        println!("continuation  - {}", rc);
        */
        // note 2: It is not enough to just not access rc, it has to be also dropped before the await

    });
}