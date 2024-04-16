pub trait Summary {
    fn summarize(&self) -> String {
        String::from("(Читать дальше...)")
    }
}

pub struct NewsArticle {
    pub headline: String,
    pub location: String,
    pub author: String,
    pub content: String
}

impl Summary for NewsArticle {
    fn summarize(&self) -> String {
        format!("{}, {} ({})", self.headline, self.author, self.location)
    }
}

pub struct Tweet {
    pub username: String,
    pub content: String,
    pub reply: bool,
    pub retweet: bool
}

impl Summary for Tweet {
    fn summarize(&self) -> String {
        format!("{}: {}", self.username, self.content)
    }
}

fn main() {
    let tweet = Tweet {
        username: String::from("horse_ebook"),
        content: String::from("конечно, как вы, наверное, уже знаете, люди"),
        reply: false,
        retweet: false
    };
    println!("1 новый твит:\n{}", tweet.summarize());
}
