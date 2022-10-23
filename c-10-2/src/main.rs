use std::{fmt::{Display, Debug}, any::{type_name, Any}};

pub trait Summary {
    fn summarize(&self) -> String;
}

pub trait SomeTrait {
    fn foo(&self) -> String {
        String::from("this is default function.")
    }
}

pub struct NewsArticle {
    pub headline: String,
    pub location: String,
    pub author: String,
    pub content: String,
}

impl Summary for NewsArticle {
    fn summarize(&self) -> String {
        format!("{}, by {} ({})", self.headline, self.author, self.location)
    }
}
impl SomeTrait for NewsArticle {}

impl Display for NewsArticle {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        Result::Ok(println!("{}", self.content))
    }
}

pub struct Tweet {
    pub username: String,
    pub content: String,
    pub reply: bool,
    pub retweet: bool,
}

impl Summary for Tweet {
    fn summarize(&self) -> String {
        format!("{}: {}", self.username, self.content)
    }
}

// pub fn notify(item: &impl Summary) {
//     println!("Breaking news! {}", item.summarize());
// }
pub fn notify<T: Summary + Display>(item: &T) {
    println!("Breaking news! {}", item.summarize());
    println!("{}", item);
}

fn some_function<T, U>(t: &T, u: &U) -> i32
where
    T: Display + Clone,
    U: Clone + Debug,
{
    let t_val: i32 = t.to_string().parse().unwrap_or(0);
    let u_val: i32 = format!("{:?}", u).to_string().parse().unwrap_or(0);

    t_val + u_val
}

fn returns_summarizable() -> impl Summary {
    NewsArticle {
        author: String::from("a"),
        content: String::from("a"),
        headline: String::from("a"),
        location: String::from("a"),
    }
}

fn main() {
    let news_article = NewsArticle {
        author: String::from("Tom"),
        content: String::from("this is news content"),
        headline: String::from("this is headline"),
        location: String::from("somewhere"),
    };

    let tweet = Tweet {
        content: String::from("this is a tweet"),
        reply: false,
        retweet: true,
        username: String::from("Sarah"),
    };

    println!("news summary is: {}", news_article.summarize());
    println!("tweet summary is: {}", tweet.summarize());
    println!("news foo is: {}", news_article.foo());
    notify(&news_article);

    let a = 10;
    let b = 20;
    println!("{}", some_function(&a, &b));
}

