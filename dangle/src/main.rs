fn main() {
  let reference_to_another = dangle();
}

fn dangle() -> &String {
  let s = String::from("hello");
  &s
}
