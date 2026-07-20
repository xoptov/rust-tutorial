fn main() {
    let v = vec![1,2,3,4,5];

    let third: &i32 = &v[2];
    println!("Третий элемент равен {}", third);

    match v.get(2) {
        Some(third) => println!("Третий элемент равен {}", third),
        None => println!("Третий элемент отсутствует.")
    }
}
