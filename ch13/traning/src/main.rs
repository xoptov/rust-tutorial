use std::thread;
use std::time::Duration;

fn simulated_expensive_calculation(intensity: u32) -> u32 {
    println!("вычисляется медленно...");
    thread::sleep(Duration::from_secs(2));
    intensity
}

fn generate_workout(intensity: u32, random_number: u32) {
    let expensive_result = simulated_expensive_calculation(intensity);
    
    if intensity < 25 {
        println!(
            "Сегодня сделайте {} отжиманий!",
            expensive_result
        );
        println!(
            "Далее, сделайте {} приседаний!",
            expensive_result
        );
    } else {
        if random_number == 3 {
            println!("Сделайте сегодня перерыв! Пейте больше воды!");
        } else {
            println!(
                "Сегодня пробежка {} минут!",
                expensive_result
            );
        }
    }
}

fn main() {
    let simulated_user_specified_value = 10;
    let simulated_random_number = 7;
    generate_workout(
        simulated_user_specified_value,
        simulated_random_number
    );
}
