use c_17_3::Post;

fn main() {
    let mut post = Post::new();

    post.add_text("I ate a salad for lunch today");
    assert_eq!("", post.content());

    post.request_review();
    assert_eq!("", post.content());
    post.add_text("test");

    post.reject();
    assert_eq!("", post.content());

    post.request_review();
    post.add_text("test");

    post.approve();
    assert_eq!("", post.content());

    post.approve();
    post.add_text("test");
    assert_eq!("I ate a salad for lunch today", post.content());
}
