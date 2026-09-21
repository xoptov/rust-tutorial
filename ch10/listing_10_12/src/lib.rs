// Объявление типажа
pub trait Summary {
    fn summarize(&self) -> String;
}

// Объявление структуры
pub struct NewsArticle {
    pub headline: String,
    pub location: String,
    pub author: String,
    pub content: String
}

// Реализация типажа для структуры
impl Summary for NewsArticle {
    fn summarize(&self) -> String {
        format!("{}, {} ({})", self.headline, self.author, self.location)
    }
}

// Обьявление структуры
pub struct Tweet {
    pub username: String,
    pub content: String, 
    pub reply: bool,
    pub retweet: bool
}

// Реализация типажа для структуры
impl Summary for Tweet {
    fn summarize(&self) -> String {
        format!("{}: {}", self.username, self.content)
    }
}
