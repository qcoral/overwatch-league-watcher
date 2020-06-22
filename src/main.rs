use webbrowser;
use enigo::*;
use std::thread::sleep_ms;
use chrono::{DateTime, Utc};

fn main() {
    let mut enigo = Enigo::new();
    let current_time: DateTime<Utc> = Utc::now();
    println!("UTC now in a custom format is: {}", current_time.format("%a %b %e %T %Y"));
    let mut formatted_time: String = format!("{}", current_time.format("%a %b %e %T %Y"));
    let mut date: String = String::from(&formatted_time);
    let mut time: String = String::from(&formatted_time);
    date.truncate(3);
    println!("{}", date);
    time.truncate(16);
    time = time.chars().rev().collect::<String>();
    time.truncate(5);
    time = time.chars().rev().collect::<String>();
    println!("{}", time);
    if date == "Sun".parse().unwrap() || date == "Sat".parse().unwrap() && time == "07:10".parse().unwrap() || time == "00:14".parse().unwrap(){
        if webbrowser::open("https://overwatchleague.com/en-us").is_ok() {
            enigo.mouse_move_to(500, 500);
            sleep_ms(3000);
            enigo.mouse_down(MouseButton::Left);
            enigo.mouse_up(MouseButton::Left);
        }
    }
}
