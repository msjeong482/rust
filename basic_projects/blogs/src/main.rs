use blogs::Post;

fn main() {
    let mut post = Post::new();

    post.add_text("i had a salad for lunch.");
    assert_eq!("", post.content());

    post.request_review();
    assert_eq!("", post.content());

    post.approve();
    assert_eq!("i had a salad for lunch.", post.content());
}
