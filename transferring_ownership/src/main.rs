fn main() {
    let _s1 = gives_ownership();
    let s2 = String::from("hello");
    let _s3 = takes_and_gives(s2);
}

fn gives_ownership() -> String {
    let some_string = String::from("hello");
    some_string
}

fn takes_and_gives(a_string: String) -> String {
    a_string
}
