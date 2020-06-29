use webbrowser;
use enigo::*;
use std::thread::sleep_ms;
use chrono::{DateTime, Utc};

fn main() {
    println!("Dari's Overwatch League watcher. Special thanks to https://gigabra.in/ for providing a platform that actually gives tokens instead of this inconsistent mess of a \"system\" that is overwatchleague.com\nSource code at https://github.com/Dari420/overwatch-league-watcher");
    loop {
        let current_time: DateTime<Utc> = Utc::now();
        println!("UTC now is: {}", current_time.format("%a %b %e %T %Y"));
        let formatted_time: String = format!("{}", current_time.format("%a %b %e %T %Y"));
        let mut date: String = String::from(&formatted_time);
        let mut time: String = String::from(&formatted_time);
        date.truncate(3);
        time.truncate(16);
        time = time.chars().rev().collect::<String>();
        time.truncate(5);
        time = time.chars().rev().collect::<String>();
        if date == String::from("Sat") && time == String::from("08:05") || time == String::from("18:35") {
            open_gigabrain_overwatch();
            break;
        }
        else if date == String::from("Sun") && time == String::from("08:05") || time == String::from("18:35") {
            open_gigabrain_overwatch();
            break;
        }
        sleep_ms(60000);
    }
}

fn open_gigabrain_overwatch() {
    let mut enigo = Enigo::new();
    println!("Opening overwatch league.");
    if webbrowser::open("https://gigabra.in").is_ok() {
        enigo.mouse_move_to(1300, 260);
        sleep_ms(2000);
        enigo.mouse_down(MouseButton::Left);
        enigo.mouse_up(MouseButton::Left);
        enigo.mouse_move_to(700, 600);
        sleep_ms(6500);
        enigo.mouse_down(MouseButton::Left);
        enigo.mouse_up(MouseButton::Left);
        sleep_ms(3000);
        enigo.mouse_down(MouseButton::Left);
        enigo.mouse_up(MouseButton::Left);
    }
}
