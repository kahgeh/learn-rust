use blogpost::Post;

fn main() {
    let mut post = Post::new();

    post.add_text("I ate a salad for lunch today");
    println!("new post, after adding text - {}", post.content());

    post.request_review();
    println!("content after request review - {}", post.content());
    assert_eq!("",post.content());

    post.approve();
    println!("content after approval - {}", post.content());
    assert_eq!("I ate a salad for lunch today", post.content())
}
