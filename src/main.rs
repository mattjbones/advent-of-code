#![feature(iter_next_chunk)]

use std::{collections::HashMap, env::args, process::exit};

pub mod day;

type DayRunner = fn();
fn main() {
    println!("Advent of code");

    let days: HashMap<String, DayRunner> = HashMap::from([
        ("1".to_string(), day::one::run as fn()),
        ("2".to_string(), day::two::run),
        ("3".to_string(), day::three::run),
        ("4".to_string(), day::four::run),
        ("5".to_string(), day::five::run),
        ("6".to_string(), day::six::run),
        ("7".to_string(), day::seven::run),
        ("8".to_string(), day::eight::run),
        ("9".to_string(), day::nine::run),
        ("10".to_string(), day::ten::run),
        ("11".to_string(), day::eleven::run),
    ]);

    let args = args().into_iter().collect::<Vec<String>>();

    if args.len() == 1 && args.first().unwrap().contains("advent_of_code") {
        println!("Select day or all");
        println!("e.g. cargo run -- 9");
        exit(0);
    }

    let selected = args[1].as_str();
    if selected == "all" {
        days.into_iter().for_each(|(_, day)| day());
    } else if let Some(day) = days.get(selected) {
        day();
    } else {
        println!("Select day no or \"all\"");
        println!("e.g. cargo run -- 9");
        exit(0);
    }
}
