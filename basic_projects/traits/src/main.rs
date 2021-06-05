pub use traits::blog::{tweet, newsarticle, Summary};

fn main() {
    let tweet = tweet::Tweet {
        username: String::from("horse_ebooks"),
        content: String::from("started learning rust language."),
        reply: false,
        retweet: false,
    };

    let news = newsarticle::NewsArticle {
        headline: String::from("head line"),
        location: String::from("Seoul"),
        author: String::from("MS"),
        content: String::from("cont"),
    };

    println!("a new tweet: {}", tweet.summarize());
    notify(tweet);
    notify(news);
}

fn notify(item: impl Summary) {
    println!("U! {}", item.summarize());
}
