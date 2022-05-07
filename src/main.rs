use webbrowser;
use enigo::*;
use std::thread::sleep_ms;
use chrono::{DateTime, Utc};
use std::io::stdin;
use std::time::Duration;

fn main() {
    let mut size = String::new();
    println!("Dari's Overwatch League watcher. yep that's pretty much it.\n
    Source code at https://github.com/dari-studios/overwatch-league-watcher\nhttps://daristudios.ca/socials if you need help");
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
        open_owl();
        if date == String::from("Fri") && time == String::from("10:15") || time == String::from("18:15") {
            open_owl();
        }
        else if date == String::from("Sat") && time == String::from("10:15") || time == String::from("18:15") {
            open_owl();
        }
        else if date == String::from("Sun") && time == String::from("10:15") || time == String::from("18:15") {
            open_owl();
        }
        sleep_ms(60000);
    }
}

fn open_owl() {
    let mut enigo = Enigo::new();
    println!("Opening overwatch league.");
    if webbrowser::open("https://overwatchleague.com/en-us/").is_ok() {
        //plays video
        enigo.mouse_move_to(500, 500);
        sleep_ms(3000);
        for x in 0..4 {
            enigo.mouse_scroll_y(-1);
            sleep_ms(20);
        }
        sleep_ms(1000);
        enigo.mouse_down(MouseButton::Left);
        sleep_ms(300);
        enigo.mouse_up(MouseButton::Left);
    }
}

fn open_contenders() {
    let mut enigo = Enigo::new();
    println!("Opening contenders");
    if webbrowser::open("https://overwatchleague.com/en-us/contenders").is_ok() {
        //plays video
        enigo.mouse_move_to(500, 500);
        sleep_ms(3000);
        for x in 0..7 {
            enigo.mouse_scroll_y(-1);
            sleep_ms(20);
        }
        sleep_ms(1000);
        enigo.mouse_down(MouseButton::Left);
        sleep_ms(300);
        enigo.mouse_up(MouseButton::Left);
    }
}