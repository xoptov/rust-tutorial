use listing_10_12::Summary;
use listing_10_12::Tweet;

fn main() {
    let tweet = Tweet {
        username: String::from("horse_ebooks"),
        content: String::from("конечно, как вы, наверное, уже знаете, люди"),
        reply: false,
        retweet: false,
    };
    println!("1 новый твит: {}", tweet.summarize());
}
