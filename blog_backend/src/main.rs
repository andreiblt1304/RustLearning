use blog_backend::Post;

fn main() {
    let mut post = Post::new();

    post.add_text("I ate mici for lunch today");

    let post = post.request_review();

    let post = post.approve();

    assert_eq!("I ate mici for lunch today", post.content());
}
