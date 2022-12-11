use std::{
    io::{Error, ErrorKind},
    ops::RangeInclusive,
};

#[derive(Debug)]
struct Cleaner(RangeInclusive<u32>);

impl TryFrom<&str> for Cleaner {
    type Error = Error;
    fn try_from(range: &str) -> Result<Self, Self::Error> {
        let range: Vec<&str> = range.split("-").collect();
        match range.len() {
            2 => {
                let start = range[0].parse::<u32>().unwrap();
                let end = range[1].parse::<u32>().unwrap();
                Ok(Self(RangeInclusive::new(start, end)))
            }
            _ => Err(Error::from(ErrorKind::InvalidInput)),
        }
    }
}

impl Cleaner {
    fn contains(&self, other: &Cleaner) -> bool {
        self.0.contains(other.0.start()) && self.0.contains(other.0.end())
    }

    fn overlap(&self, other: &Cleaner) -> bool {
        self.0.contains(other.0.start()) || self.0.contains(other.0.end())
    }
}

pub fn run() {
    println!("Day 4");

    {
        println!("Part 1 / 2");
        let mut total = 0;
        for line in include_str!("input").lines() {
            let pair: Vec<Cleaner> = line
                .split(",")
                .map(|set| Cleaner::try_from(set).unwrap())
                .collect();

            if pair[0].contains(&pair[1]) || pair[1].contains(&pair[0]) || pair[0].overlap(&pair[1])
            {
                total += 1;
            }
        }

        println!("Total: {total}");
    }
}
