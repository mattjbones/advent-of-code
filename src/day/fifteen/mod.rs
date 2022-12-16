use std::collections::HashMap;

use position::*;
mod position;

pub fn run() {
    println!("Day 15");

    {
        println!("Part 1 - Sample");
        let result = part_1(include_str!("sample"), 10);
        println!(" Result {result}");
        println!(" Expected 26");
    }

    // {
    //     println!("Part 1 - input");
    //     let result = part_1(include_str!("input"), 2_000_000);
    //     println!(" Result {result}");
    //     println!(" Expected ...");
    // }
}

const SENSOR: &str = "Sensor at ";
const BEACON: &str = " closest beacon is at ";
const X: &str = "x=";
const Y: &str = "y=";

fn part_1(input: &str, y_index: isize) -> usize {
    let coords: HashMap<String, Position> = parse_input(input);
    // print_grid(&coords, false);
    let coverage: Vec<Position> = build_coverage(&coords, y_index);
    analyse_coverage(&coverage, y_index)
}

fn build_coverage_for_target(
    coords: &HashMap<String, Position>,
    target: &Position,
    _: isize,
) -> HashMap<String, Position> {
    let mut coverage = coords.clone();
    let mut level = 1;
    println!("building coverage");
    loop {
        // lines
        let adjustment = level * 1;
        let lower_x = dbg!(target.x - adjustment);
        let upper_x = dbg!(target.x + adjustment);

        let lower_y = dbg!(target.y - adjustment);
        let upper_y = dbg!(target.y + adjustment);
        println!("{}", target);

        for y in lower_y..upper_y {
            for x in lower_x..upper_x {
                if x == target.x && y == target.y {
                    continue;
                }
                coverage.insert(format!("{x},{y}"), Position::new(x, y, Item::Coverage));
                println!("{x},{y}");
            }
        }

        print_map(&coverage, false);

        if level == 1 {
            break;
        }

        level += 1;
    }
    coverage
}

fn build_coverage(coords: &HashMap<String, Position>, _: isize) -> Vec<Position> {
    // find 8,7
    // let mut coverage = HashMap::new();
    // println!(
    //     "beacons {:#?}",
    //     coords
    //         .iter()
    //         .filter(|(_, item)| item.is_beacon())
    //         .collect::<Vec<(&String, &Position)>>()
    // );
    // println!(
    //     "sensors {:#?}",
    //     coords
    //         .iter()
    //         .filter(|(_, item)| item.is_sensor())
    //         .collect::<Vec<(&String, &Position)>>()
    // );

    // println!("{:#?}", coords);
    // print_map(coords, false);
    let mut coverage = coords.clone();
    let target = coords.get("8,7").unwrap();
    let coverage_for_target = &mut build_coverage_for_target(&coords, target, -1);
    coverage_for_target.iter().for_each(|(key, val)| {
        coverage.insert(key.clone(), val.clone());
    });
    print_map(&coverage, false);
    coverage
        .into_iter()
        .map(|(_, x)| x)
        .collect::<Vec<Position>>()

    // let rc_coords = Arc::new(coords.clone());
    // let results: Vec<Position> = coords
    //     .clone()
    //     .into_iter()
    //     .filter(|(_, item)| item.is_sensor())
    //     .map(|(_, item)| {
    //         let local = Arc::clone(&rc_coords);
    //         std::thread::spawn(move || {
    //             build_coverage_for_target(&local, &item, y_index)
    //                 .iter()
    //                 .map(|(_, val)| val.clone())
    //                 .collect::<Vec<Position>>()
    //         })
    //     })
    //     .map(|handle| handle.join())
    //     .flat_map(|res| match res {
    //         Ok(val) => val,
    //         Err(e) => panic!("{:#?}", e),
    //     })
    //     .collect::<Vec<Position>>();

    // let coverage: HashSet<Position> = HashSet::from_iter(results);
    // print_grid(
    //     &coverage.clone().into_iter().collect::<Vec<Position>>(),
    //     false,
    // );
    // coverage.clone().into_iter().collect::<Vec<Position>>()
}

fn analyse_coverage(coords: &Vec<Position>, y_index: isize) -> usize {
    coords
        .iter()
        .filter(|val| val.y == y_index && val.item == Item::Coverage)
        .collect::<Vec<&Position>>()
        .len()
}

fn parse_position(i: usize, str: &str) -> Position {
    let working_string = if i == 0 {
        &str[SENSOR.len()..]
    } else {
        let beacon_start_index = str.find(BEACON).unwrap();
        &str[(beacon_start_index + BEACON.len())..]
    };

    let part = working_string
        .replace(" ", "")
        .split(",")
        .enumerate()
        .map(|(i, part)| {
            let no_part = if i == 0 {
                &part[X.len()..]
            } else {
                &part[Y.len()..]
            };
            no_part.parse::<isize>().unwrap()
        })
        .collect::<Vec<isize>>();

    Position::new(
        part[0],
        part[1],
        if i == 0 { Item::Sensor } else { Item::Beacon },
    )
}

fn parse_input(input_str: &str) -> HashMap<String, Position> {
    let mut position_map: HashMap<String, Position> = HashMap::new();
    input_str.lines().for_each(|line| {
        line.split(":")
            .enumerate()
            .map(|(i, str)| parse_position(i, str))
            .for_each(|pos| {
                position_map.insert(format!("{},{}", pos.x, pos.y).to_string(), pos);
            });
    });
    position_map
}

fn print_map(position_map: &HashMap<String, Position>, clear: bool) {
    let positions = position_map
        .iter()
        .map(|(_, item)| item.to_owned())
        .collect::<Vec<Position>>();
    print_grid(&positions, clear);
}

fn print_grid(positions: &Vec<Position>, clear: bool) {
    if positions.len() == 0 {
        // println!("empty");
        return;
    }

    if clear {
        print!("\x1B[2J\x1B[1;1H");
    }
    // println!("{positions:#?}");

    let max_x = positions.iter().max_by(|x, y| x.x.cmp(&y.x)).unwrap().x + 2;
    let min_x = positions.iter().min_by(|x, y| x.x.cmp(&y.x)).unwrap().x - 1;

    let max_y = positions.iter().max_by(|x, y| x.y.cmp(&y.y)).unwrap().y + 2;
    let min_y = positions.iter().min_by(|x, y| x.y.cmp(&y.y)).unwrap().y;

    // println!("max_x({max_x}) min_x({min_x})");
    for i in 0..=3 {
        println!();
        for grid_x in min_x..max_x {
            let chars = grid_x.to_string().chars().collect::<Vec<char>>();
            if grid_x == 0 {
                print!("{}", String::from_iter(vec![" "; (min_x * -1) as usize]));
            }
            if grid_x == 0 && i == 0 {
                print!("  ");
            }
            if grid_x == 0 && i == 1 {
                print!("0");
            } else if grid_x % 5 == 0 {
                if chars.len() == 1 && i == 1 {
                    print!("{}", chars[i - 1]);
                } else if chars.len() > 1 && i < chars.len() {
                    print!("{}", chars[i]);
                }
            } else {
                print!(" ");
            }
        }
    }

    println!();
    for grid_y in min_y..max_y {
        if grid_y < 10 && grid_y >= 0 {
            print!("  {} ", grid_y);
        } else if grid_y <= -10 {
            print!("{} ", grid_y);
        } else {
            print!(" {} ", grid_y);
        }
        for grid_x in min_x..max_x {
            let cord = positions
                .iter()
                .position(|item| item.x == grid_x && item.y == grid_y);
            if let Some(cord) = cord {
                let pos = &positions[cord];
                match pos.item {
                    Item::Beacon => print!("B"),
                    Item::Sensor => print!("S"),
                    Item::Coverage => print!("#"),
                }
            } else {
                print!(".");
            }
        }
        print!("\n");
    }
    println!();
}
