use crate::runner::Runner;

struct Node {
    name: String,
    rate: usize,
    linked: Vec<String>,
}

pub struct Sixteen {}
// 30 mins
// 1 minute to open
// 1 minute from one tunnel to next
// start at AA - no flow rate / damaged
impl Runner for Sixteen {
    fn run_all(&self, sample: &str, input: &str) {
        println!("Day 16");
        self.part_1_sample(sample);
    }

    fn part_1_sample(&self, sample: &str) {
        println!("Part 1 - Sample");
        sample.lines().map(|line| {
            parse_line(line);
        });
        let result: usize = 0;
        println!(" Result {result}");
        println!(" Expected 1651")
    }

    fn part_1_input(&self, input: &str) {
        todo!()
    }

    fn part_2_sample(&self, sample: &str) {
        todo!()
    }

    fn part_2_input(&self, input: &str) {
        todo!()
    }
}

const VALVE: &str = "Valve ";
const RATE: &str = "flow rate=";
const LINKED: &str = "lead to valves";

fn parse_line(line: &str) {
    let name = &line[VALVE.len()..3];
    println!("{name}");
}
