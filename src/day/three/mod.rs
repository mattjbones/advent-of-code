use std::env::current_dir;
use std::fs::File;
use std::io::{self, BufRead, Error};
use std::ops::RangeInclusive;

const INPUT: &str = "/src/day/three/input";

pub fn run() -> Result<(), Error> {
    println!("Day 3");
    let path = current_dir()?;
    let priorities = generate_priorities();

    {
        println!("Part 1");

        let file = File::open(format!("{}{}", path.display(), INPUT))?;
        let lines = io::BufReader::new(&file).lines();

        let mut matches: Vec<char> = Vec::new();
        for line in lines {
            if let Ok(line) = line {
                let midpoint = line.len() / 2;
                let first_section = &line[0..midpoint];
                let second_section = &line[midpoint..line.len()];
                let mut local_matches = Vec::new();
                for first in first_section.chars() {
                    if second_section.contains(first) && !local_matches.contains(&first) {
                        local_matches.push(first)
                    }
                }
                matches.extend(local_matches.iter());
            }
        }

        let total = matches
            .iter()
            .map(|c| priorities.iter().position(|x| x == c).unwrap())
            .reduce(|acc, val| acc + val)
            .unwrap();
        println!("Total: {:?}", total);
    }

    {
        println!("Part 2");

        let file = File::open(format!("{}{}", path.display(), INPUT))?;
        let lines = io::BufReader::new(&file).lines();

        let mut badges: Vec<char> = Vec::new();
        let mut group: Vec<String> = Vec::new();
        for line in lines {
            if let Ok(line) = line {
                group.push(line);
            }

            if group.len() == 3 {
                let mut matched: char = 0 as char;
                group.sort_by(|a, b| b.len().cmp(&a.len()));
                let longest = group.pop().unwrap();
                for char in longest.chars() {
                    let letters: Vec<Option<char>> = group
                        .iter()
                        .map(|line| line.chars().find(|c| c == &char))
                        .filter(|l| l.is_some())
                        .collect();
                    if letters.len() == 2 {
                        matched = letters[0].unwrap();
                    }
                }

                badges.push(matched);
                group.clear();
            }
        }

        let total = badges
            .iter()
            .map(|c| priorities.iter().position(|x| x == c).unwrap())
            .reduce(|acc, val| acc + val)
            .unwrap();
        println!("Badges total: {:?}", total);
    }
    Ok(())
}

fn generate_priorities() -> [char; 53] {
    let mut priorities: [char; 53] = [0 as char; 53];
    let mut count = 27;
    let start: u8 = 65;
    let end: u8 = 90;
    for num in RangeInclusive::new(start, end) {
        let char = num as char;
        priorities[count] = char;
        priorities[count - 26] = char.to_ascii_lowercase();
        count += 1;
    }

    priorities
}
