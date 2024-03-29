fn main() {
// example 1:
//  loop {
//    println!("Ещё раз");
//  }

// example 2:
//  let mut counter = 0;
//  let result = loop {
//    counter += 1;
//    if counter == 10 {
//      break counter * 2;
//    }
//  };
//  println!("Результат равен {}", result);

// example 3:
//  let mut number = 3;
//  while number != 0 {
//      println!("{}!", number);
//      number = number - 1;
//  }
//  println!("Поехали!!!");

// example 4:
//  let a = [10,20,30,40,50];
//  let mut index = 0;
//  while index < 5 {
//    println!("Значение равно {}", a[index]);
//    index = index + 1;
//  }

// example 5:
//  let a = [10,20,30,40,50];
//  for element in a.iter() {
//    println!("Значение равно {}", element);
//  }

// example 6:
  for number in (1..4).rev() {
    println!("Значение равно {}", number);
  }
}
