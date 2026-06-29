struct ImportantExcerpt<'a> {
    part: &'a str
}

fn main() {
    let novel = String::from("Зовите меня измаил, несколько лет тому назад...");
    let first_sentence = novel.split('.')
        .next()
        .expect("Не смог отыскать '.'");
    
    let i = ImportantExcerpt { part: first_sentence };
}
