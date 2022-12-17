use std::collections::HashMap;

use crate::runner::Runner;

#[derive(Debug, Clone, Hash)]
struct Node {
    name: String,
    rate: usize,
    is_open: bool,
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
        // self.part_1_input(input);
    }

    fn part_1_sample(&self, sample: &str) {
        println!("Part 1 - Sample");

        let nodes = sample
            .lines()
            .map(|line| parse_line(line))
            .collect::<Vec<(String, Node)>>();

        let node_map: HashMap<String, Node> = HashMap::from_iter(nodes);
        let result = walk_map(&node_map, "AA", 30);

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

fn walk_map(nodes: &HashMap<String, Node>, start: &str, time_limit: usize) -> usize {
    println!();

    let working_nodes = &mut nodes.clone();
    let mut current_key = start;
    let mut total_pressure = 0;
    let mut open_valves: Vec<String> = Vec::new();
    let mut visited_valves: Vec<String> = vec![start.to_string()];
    for min in 1..=time_limit {
        let pressure_from_open_valves = check_open_valves(&nodes, &open_valves);
        total_pressure += pressure_from_open_valves;
        println!("== Minute {min} ==");
        if open_valves.len() > 1 {
            println!(
                "Valve {} are open, releasing {} pressure.",
                open_valves.clone().join(", "),
                pressure_from_open_valves
            )
        } else if open_valves.len() > 0 {
            println!(
                "Valve {} is open, releasing {} pressure.",
                open_valves.clone().join(", "),
                pressure_from_open_valves
            )
        } else {
            println!("No valves are open.");
        }

        let current_node = working_nodes.get(current_key).unwrap();
        if (current_node.rate == 0 || open_valves.contains(&current_key.to_string()))
            && visited_valves.contains(&current_key.to_string())
        {
            // move node
            // println!("current: {current_node:#?}");
            visited_valves.push(current_key.to_string());
            current_key = working_nodes
                .iter()
                .filter(|entry| {
                    current_node.linked.contains(&entry.0)
                        && (!open_valves.contains(&entry.0) || visited_valves.contains(&entry.0))
                })
                .max_by(|(_, node1), (_, node2)| node1.rate.cmp(&node2.rate))
                .unwrap()
                .0;
            println!("You move to valve {}.", &current_key);
        } else if !open_valves.contains(&current_key.to_string()) {
            open_valves.push(current_key.to_string());
            println!("You open valve {}.", &current_key)
            // working_nodes
            //     .entry(current_key.to_string())
            //     .and_modify(|node| node.is_open = true);
        } else {
        }

        println!();

        // if min == 4 {
        //     break;
        // }
    }

    total_pressure
}

fn check_open_valves(nodes: &HashMap<String, Node>, open_valves: &Vec<String>) -> usize {
    nodes.iter().fold(0, |acc, (key, node)| {
        if open_valves.contains(key) {
            acc + node.rate
        } else {
            acc
        }
    })
}

const VALVE: &str = "Valve ";
const LINKED: &str = "to valve";

fn parse_line(line: &str) -> (String, Node) {
    let name = &line[VALVE.len()..(VALVE.len() + 3)].trim();
    let rate = &line
        .split("=")
        .enumerate()
        .map(|(i, part)| {
            if i == 1 {
                part.split(";").collect::<Vec<&str>>()[0].trim()
            } else {
                ""
            }
        })
        .collect::<String>();

    let linked_split = &line.split(LINKED).collect::<Vec<&str>>()[1].replace('s', "");
    let linked = linked_split
        .split(", ")
        .map(|item| item.trim().to_string())
        .collect::<Vec<String>>();

    (
        name.to_string(),
        Node {
            name: name.to_string(),
            rate: rate.parse::<usize>().unwrap(),
            linked: linked.to_vec(),
            is_open: false,
        },
    )
}
