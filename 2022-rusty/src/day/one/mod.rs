use std::env;
use std::fs;

const INPUT: &str = "/src/day/one/input";

pub fn run() {
    println!("Day one");
    let path = env::current_dir().unwrap();
    let totals = fs::read_to_string(format!("{}{}", path.display(), INPUT))
        .expect("Should have been able to read the file");

    let per_elfs_str: Vec<&str> = totals.split("\n\n").collect();
    let per_elfs_vec: Vec<Vec<&str>> = per_elfs_str
        .iter()
        .map(|per_elf| per_elf.split("\n").collect::<Vec<&str>>())
        .collect();

    let mut totals: Vec<i32> = Vec::new();
    for per_elf in per_elfs_vec {
        let elf_total = per_elf
            .iter()
            .map(|e| e.parse::<i32>().unwrap())
            .reduce(|acc, total| acc + total);

        if let Some(some_elf_total) = elf_total {
            totals.push(some_elf_total);
        }
    }

    totals.sort();
    totals.reverse();
    totals.truncate(3);
    let mut total = 0;
    for per_elf_total in &totals {
        total += per_elf_total;
    }

    println!("biggest total: {:}, {:?}\n", total, totals);
}
