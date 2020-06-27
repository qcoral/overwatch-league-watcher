use webbrowser;
use enigo::*;
use std::thread::sleep_ms;
use chrono::{DateTime, Utc};

fn main() {
    let mut enigo = Enigo::new();
    loop {
        let current_time: DateTime<Utc> = Utc::now();
        println!("UTC now is: {}", current_time.format("%a %b %e %T %Y"));
        let formatted_time: String = format!("{}", current_time.format("%a %b %e %T %Y"));
        let mut date: String = String::from(&formatted_time);
        let mut time: String = String::from(&formatted_time);
        date.truncate(3);
        println!("{}", date);
        time.truncate(16);
        time = time.chars().rev().collect::<String>();
        time.truncate(5);
        time = time.chars().rev().collect::<String>();
        println!("{}", time);
        if date == String::from("Sun") || date == String::from("Sat") && time == String::from("07:10") || time == String::from("17:10") {
            println!("time!");
            if webbrowser::open("https://overwatchleague.com/en-us").is_ok() {
                enigo.mouse_move_to(500, 500);
                sleep_ms(3000);
                enigo.mouse_down(MouseButton::Left);
                enigo.mouse_up(MouseButton::Left);
            }
        }
        sleep_ms(30000);
    }
}
