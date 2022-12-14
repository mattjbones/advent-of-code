#![feature(iter_next_chunk)]

use std::{env::args, process::exit};

pub mod day;

type DayRunner = fn();
fn main() {
    println!("Advent of code");

    let days: Vec<DayRunner> = Vec::from([
        day::one::run as fn(),
        day::two::run,
        day::three::run,
        day::four::run,
        day::five::run,
        day::six::run,
        day::seven::run,
        day::eight::run,
        day::nine::run,
        day::ten::run,
        day::eleven::run,
        day::twelve::run,
        day::thirteen::run,
        day::fourteen::run,
    ]);

    let args = args().into_iter().collect::<Vec<String>>();

    if args.len() == 1 && args.first().unwrap().contains("advent_of_code") {
        println!("Select day or all");
        println!("e.g. cargo run -- 9");
        exit(0);
    }

    let selected = args[1].as_str();
    if selected == "all" {
        days.into_iter().for_each(|day| day());
    } else if let Ok(selected_no) = selected.parse::<usize>() {
        if selected_no > days.len() {
            println!("Day not valid");

            println!("Select day no or \"all\"");
            println!("e.g. cargo run -- 9");
            exit(0);
        }
        days[selected_no - 1]();
    } else {
        println!("Select day no or \"all\"");
        println!("e.g. cargo run -- 9");
        exit(0);
    }
}
