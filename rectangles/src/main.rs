struct Rectangle {
  width: u32,
  height: u32
}

fn main() {
  let rect = Rectangle {width: 30, height: 50};
  println!("Площадь прямоугольника равна {} квадратным пикселам.", area(&rect));
}

fn area(rectangle: &Rectangle) -> u32 {
  rectangle.width * rectangle.height
}

