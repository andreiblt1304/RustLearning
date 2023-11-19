fn main() {
    let mut post = Post::new();

    post.add_text("I ate mici for lunch today");
    assert_eq!("", post.content());

    post.request_review();
    assert_eq!("", post.content());

    post.approve();
    assert_eq!("I ate mici for lunch today", post.content());
}
