struct ImportantExcerpt<'a> {
    part: &'a str,
}

fn main() {
    let novel = String::from("Зовите меня Измаил. Несколько лет тому назад...");
    let first_sentence = novel.split('.').next().except("Не смог отыскать '.'");
    let i = ImportantExcerpt {part: first_sentence};
}
