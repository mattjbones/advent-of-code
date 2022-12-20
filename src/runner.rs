use std::{process::exit, time::SystemTime};

pub trait Runner<T>
where
    T: std::fmt::Display,
    T: std::cmp::PartialEq,
{
    fn run_all(&self, sample: &str, input: &str) {
        {
            println!();
            println!("Part 1 - Sample");
            let start = SystemTime::now();
            let (result, expected) = self.part_1_sample(sample);
            let end = SystemTime::now();
            println!(" Result   {result}");
            println!(" Expected {expected}");
            println!("  (took: {:?})", end.duration_since(start).unwrap());
            if result != expected {
                println!("\n--- incorrect ---");
                exit(1);
            }
        }

        {
            println!();
            println!("Part 1 - Input");
            let start = SystemTime::now();
            let (result, expected) = self.part_1_input(input);
            let end = SystemTime::now();
            println!(" Result   {result}");
            println!(" Expected {expected}");
            println!("  (took: {:?})", end.duration_since(start).unwrap());
            if result != expected {
                println!("\n--- incorrect ---");
                exit(1);
            }
        }

        {
            println!();
            println!("Part 2 - Sample");
            let start = SystemTime::now();
            let (result, expected) = self.part_2_sample(sample);
            let end = SystemTime::now();
            println!(" Result   {result}");
            println!(" Expected {expected}");
            println!("  (took: {:?})", end.duration_since(start).unwrap());
            if result != expected {
                println!("\n--- incorrect ---");
                exit(1);
            }
        }

        {
            println!();
            println!("Part 2 - Input");
            let start = SystemTime::now();
            let (result, expected) = self.part_2_input(input);
            let end = SystemTime::now();
            println!(" Result   {result}");
            println!(" Expected {expected}");
            println!("  (took: {:?})", end.duration_since(start).unwrap());
            if result != expected {
                println!("\n--- incorrect ---");
                exit(1);
            }
        }
    }

    fn part_1_sample(&self, sample: &str) -> (T, T);
    fn part_1_input(&self, input: &str) -> (T, T);
    fn part_2_sample(&self, sample: &str) -> (T, T);
    fn part_2_input(&self, input: &str) -> (T, T);
}
