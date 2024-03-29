mod back_of_house {
  pub struct Breakfast {
    pub toast: String,
    seasonal_fruit: String
  }
  impl Breakfast {
    pub fn summer(toast: &str) -> Breakfast {
      Breakfast {
        toast: String::from(toast),
        seasonal_fruit: String::from("персики")
      }
    }
  }
}

pub fn eat_in_restaurant() {
  // Заказать летом завтрак с ржаным тостом
  let mut meal = back_of_house::Breakfast::summer("ржаной");
  // Изменить мнение о том, какой хлеб мы хотели
  meal.toast = String::from("пшеничный");
  println!("Я бы хотел {} тост, пожалуйста", meal.toast);
}

fn main() {

}
