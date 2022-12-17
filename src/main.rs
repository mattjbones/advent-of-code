#![feature(iter_next_chunk)]

use std::{env, env::args, fs, process::exit};

use crate::runner::Runner;

pub mod day;
pub mod runner;

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
        day::fifteen::run,
    ]);

    let other_days: Vec<(Box<dyn Runner<_>>, String)> = Vec::from([
        (
            Box::new(day::sixteen::Sixteen {}) as Box<dyn Runner<_>>,
            "sixteen".to_string(),
        ),
        (
            Box::new(day::seventeen::Seventeen {}) as Box<dyn Runner<_>>,
            "seventeen".to_string(),
        ),
    ]);

    let args = args().into_iter().collect::<Vec<String>>();

    if args.len() == 1 && args.first().unwrap().contains("advent_of_code") {
        println!("Select day or all");
        println!("e.g. cargo run -- 9");
        exit(0);
    }

    let selected = args[1].as_str();
    if selected == "all" {
        let days_total = days.len();
        days.into_iter().for_each(|day| day());
        other_days
            .iter()
            .enumerate()
            .for_each(|(i, day_runner)| run_day(&day_runner, &format!("{}", i + days_total)));
    } else if let Ok(selected_no) = selected.parse::<usize>() {
        if selected_no > days.len() && selected_no - days.len() > other_days.len() {
            println!("Day not valid");

            println!("Select day no or \"all\"");
            println!("e.g. cargo run -- 9");
            exit(0);
        }

        if let Some(day) = days.get(selected_no - 1) {
            day();
        } else {
            let day_runner = other_days.get(selected_no - days.len() - 1).unwrap();
            run_day(day_runner, selected);
        }
    } else {
        println!("Select day no or \"all\"");
        println!("e.g. cargo run -- 9");
        exit(0);
    }
}

fn run_day<T: std::fmt::Display>((runner, path): &(Box<dyn Runner<T>>, String), selected: &str) {
    let dir = env::current_dir().unwrap();
    let sample_path = format!("{}/src/day/{path}/sample", dir.display());
    let input_path = format!("{}/src/day/{path}/input", dir.display());

    let sample_string = fs::read_to_string(sample_path).unwrap();
    let input_string = fs::read_to_string(input_path).unwrap();

    println!("Day {selected}");
    runner.run_all(&sample_string, &input_string);
}
