use position::*;
mod position;

pub fn run() {
    println!("Day 15");

    {
        println!("Part 1 - Sample");
        let result = part_1(include_str!("sample"));
        println!(" Result {result}");
        println!(" Expected ");
    }
}

const SENSOR: &str = "Sensor at ";
const BEACON: &str = " closest beacon is at ";
const X: &str = "x=";
const Y: &str = "y=";

fn part_1(input: &str) -> isize {
    let coords = parse_input(input);
    print_grid(&coords, false);
    0
}

fn parse_input(input_str: &str) -> Vec<Position> {
    input_str
        .lines()
        .flat_map(|line| {
            line.split(":").enumerate().map(|(i, str)| {
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
            })
        })
        .collect::<Vec<Position>>()
}

fn print_grid(positions: &Vec<Position>, clear: bool) {
    if clear {
        print!("\x1B[2J\x1B[1;1H");
    }
    // println!("{positions:#?}");

    let max_x = positions.iter().max_by(|x, y| x.x.cmp(&y.x)).unwrap().x + 2;
    let min_x = positions.iter().min_by(|x, y| x.x.cmp(&y.x)).unwrap().x - 1;

    let max_y = positions.iter().max_by(|x, y| x.y.cmp(&y.y)).unwrap().y + 2;
    let min_y = positions.iter().min_by(|x, y| x.y.cmp(&y.y)).unwrap().y;

    println!("max_x({max_x}) min_x({min_x})");
    for i in 0..=1 {
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
                // if chars.len() == 1 && i == 1 {
                //     print!(" ");
                // } else if chars.len() > 1 && i < chars.len() {
                //     print!(" ");
                // }else
            }
        }
    }
    println!();
    for grid_y in min_y..max_y {
        if grid_y < 10 {
            print!(" {} ", grid_y);
        } else {
            print!("{} ", grid_y);
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
                }
            } else {
                print!(".");
            }
        }
        print!("\n");
    }
    println!();
}
