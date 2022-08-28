extern crate colorful;

use colorful::Color;
use colorful::Colorful;


pub fn log(x: &str, y: &str){

    match y {
        "init" => println!("{}",x.color(Color::Yellow).bold()),
        "ok" => println!("{}",x.color(Color::Green).bold()),
        "err" => println!("{}",x.color(Color::Red).bold()),
        "rain" => x.rainbow(),
        _ => println!("{}",x.color(Color::White).bold()),
    }
}