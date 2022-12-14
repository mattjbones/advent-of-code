mod position;

use position::*;

pub fn run() {
    println!("Day 14");

    // {
    //     println!("Part 1 - Sample");
    //     let result = part_1(include_str!("sample"));
    //     println!(" Result {result}");
    //     println!(" Expected 24");
    // }

    // {
    //     println!("Part 1 - Input");
    //     let result = part_1(include_str!("input"));
    //     println!(" Result {result}");
    //     println!(" Expected 828");
    // }

    {
        println!("Part 2 - Sample");
        let result = part_2(include_str!("sample"));
        println!(" Result {result}");
        println!(" Expected 93");
    }

    {
        println!("Part 2 - Input");
        let result = part_2(include_str!("input"));
        println!(" Result {result}");
        println!(" Expected ...");
    }
}

fn part_1(input: &str) -> u32 {
    let start_position = Position::new(500, 0, Item::Start);
    println!("Start {start_position}");
    let mut wall_points = parse_path_points(input);
    wall_points.push(start_position);

    let max_y = wall_points.iter().max_by(|x, y| x.y.cmp(&y.y)).unwrap().y;
    print_grid(&wall_points, false);

    simulate_sand(&wall_points, max_y)
}

fn part_2(input: &str) -> u32 {
    let start_position = Position::new(500, 0, Item::Start);
    println!("Start {start_position}");
    let mut wall_points = parse_path_points(input);
    wall_points.push(start_position);

    add_floor(&mut wall_points);
    print_grid(&wall_points, false);
    let max_y = wall_points.iter().max_by(|x, y| x.y.cmp(&y.y)).unwrap().y;

    simulate_sand(&wall_points, max_y)
}

fn add_floor(wall_points: &mut Vec<Position>) {
    let max_y = wall_points.iter().max_by(|x, y| x.y.cmp(&y.y)).unwrap().y + 2;

    let max_x = wall_points.iter().max_by(|x, y| x.x.cmp(&y.x)).unwrap().x + max_y * 7 / 8;
    let min_x = wall_points.iter().min_by(|x, y| x.x.cmp(&y.x)).unwrap().x - max_y * 7 / 8;

    for i in min_x..max_x {
        wall_points.push(Position::new(i, max_y, Item::Wall));
    }
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

    // println!("x(min,max) ({},{})", min_x, max_x);
    // println!("y(min,max) ({},{})", min_y, max_y);
    // for i in 0..3 {
    //     for grid_x in min_x..max_x {
    //         if i == 0 && grid_x == 0 {

    //         }
    //     }
    // }
    for grid_y in min_y..max_y {
        print!("({})", grid_y);
        for grid_x in min_x..max_x {
            let cord = positions
                .iter()
                .position(|item| item.x == grid_x && item.y == grid_y);
            if let Some(cord) = cord {
                let pos = &positions[cord];
                match pos.item {
                    Item::Sand => print!("o"),
                    Item::Wall => print!("#"),
                    Item::Start => print!("+"),
                    Item::Settle => print!("~"),
                }
            } else {
                print!(".");
            }
        }
        print!("\n");
    }
}

fn parse_path_points(input: &str) -> Vec<Position> {
    let path_points = input
        .lines()
        .into_iter()
        .flat_map(|line| {
            let mut positions: Vec<Position> = Vec::new();
            let line_pos = line
                .split(" -> ")
                .map(|pos| {
                    let pos_split = pos.split(",").collect::<Vec<&str>>();
                    Position::new(
                        pos_split[0].parse::<isize>().unwrap(),
                        pos_split[1].parse::<isize>().unwrap(),
                        Item::Wall,
                    )
                })
                .collect::<Vec<Position>>();

            for (i, pos) in line_pos.iter().enumerate() {
                if i + 1 < line_pos.len() {
                    let next_pos = &line_pos[i + 1];
                    positions.push(pos.clone());
                    positions.append(&mut pos.get_points_between(next_pos));
                }
            }
            positions
        })
        .collect::<Vec<Position>>();

    path_points
}

fn simulate_sand(wall_points: &Vec<Position>, max_y: isize) -> u32 {
    println!("simulating...");

    let mut sand_grid = wall_points.clone();
    let mut count = 0;
    let start_sand = Position::new(500, 0, Item::Sand);
    loop {
        let sand = start_sand.clone();
        sand_grid.push(sand);
        let finished = loop {
            //fall
            let reference_grid = &sand_grid.clone();
            let settle_grid = &mut sand_grid.clone();
            let sand = sand_grid.last_mut().unwrap();

            // println!("new sand {:?}", sand);
            let potential_move = sand.clone().fall();
            if check_collision(&potential_move, reference_grid) {
                let position = settle(sand, reference_grid, settle_grid, max_y);
                // print_grid(&settle_grid, true);
                // println!("new_pos {:?}", &position);
                if let Some(settled) = position.clone() {
                    sand.x = settled.x;
                    sand.y = settled.y;
                    count += 1;
                }

                if next_row_sand(&start_sand, &reference_grid) {
                    break true;
                }

                // print_grid(&sand_grid, true);
                break position.is_none();
            }

            sand.fall();
            // print_grid(&sand_grid, true);
            // std::thread::sleep(std::time::Duration::from_millis(16));
        };

        if finished {
            break;
        }
    }
    print_grid(&sand_grid, false);
    count
}

fn check_collision(sand: &Position, grid: &Vec<Position>) -> bool {
    let next_position = grid.iter().find(|pos| pos.x == sand.x && pos.y == sand.y);
    if let Some(next) = next_position {
        next.item == Item::Wall || next.item == Item::Sand
    } else {
        false
    }
}

fn next_row_sand(sand: &Position, grid: &Vec<Position>) -> bool {
    let mut left = sand.clone();
    left.x -= 1;

    let mut right = sand.clone();
    right.x += 1;

    let options = vec![sand, &left, &right];
    let positions = options
        .iter()
        .map(|opt| grid.iter().find(|pos| pos.x == opt.x && pos.y == opt.y + 1))
        .collect::<Vec<Option<&Position>>>();

    positions.iter().all(|item| {
        if let Some(item) = item {
            item.is_sand()
        } else {
            false
        }
    })
}

fn settle(
    sand: &Position,
    grid: &Vec<Position>,
    settle_grid: &mut Vec<Position>,
    max_y: isize,
) -> Option<Position> {
    // println!("settle");
    // print_grid(&settle_grid, true);

    let mut left = sand.clone();
    left.x -= 1;

    let mut right = sand.clone();
    right.x += 1;

    let options = vec![sand, &left, &right];
    let positions = options
        .iter()
        .map(|opt| grid.iter().find(|pos| pos.x == opt.x && pos.y == opt.y + 1))
        .collect::<Vec<Option<&Position>>>();

    let any_start = positions.iter().any(|item| {
        if let Some(item) = item {
            item.is_start()
        } else {
            false
        }
    });
    if any_start {
        panic!("lol");
    }

    let all_none = positions.iter().all(|item| item.is_none());
    if all_none {
        // println!("sand {}", sand);
        // println!("options {:?}", options);
        // println!("positions {:?}", positions);
        // println!("all none");

        let mut final_pos = sand.clone();
        final_pos.item = Item::Settle;
        settle_grid.push(final_pos);
        // print_grid(settle_grid, true);

        if sand.y < max_y + 2 {
            return settle(&sand.clone().fall(), grid, settle_grid, max_y);
        } else {
            return None;
        }
    }

    let all_walls_or_sand = positions.iter().all(|item| {
        if let Some(item) = item {
            item.is_wall()
        } else {
            false
        }
    }) || positions.iter().all(|item| {
        if let Some(item) = item {
            item.is_sand()
        } else {
            false
        }
    }) || positions.iter().all(|item| {
        if let Some(item) = item {
            item.is_sand() || item.is_wall()
        } else {
            false
        }
    });

    if all_walls_or_sand {
        // println!("all walls / sand");
        let mut final_pos = sand.clone();
        final_pos.item = Item::Settle;
        settle_grid.push(final_pos);
        // print_grid(settle_grid, true);
        return Some(sand.clone());
    }

    let is_empty_wall_sand = positions
        .iter()
        .enumerate()
        .map(|(i, item)| match i {
            0 => {
                if let Some(item) = item {
                    item.is_wall()
                } else {
                    false
                }
            }
            1 => item.is_none(),
            2 => {
                if let Some(item) = item {
                    item.is_sand()
                } else {
                    false
                }
            }
            _ => panic!("ded"),
        })
        .all(|x| x);

    if is_empty_wall_sand {
        // println!("empty_wall_sand");
        // println!("start - position {:?}", sand);
        // println!("positions: {:?}", positions);
        // println!("proposed - position {:?}", left.clone().fall());
        let mut final_pos = left.clone();
        final_pos.item = Item::Settle;
        settle_grid.push(final_pos);
        return settle(&left.clone().fall(), grid, settle_grid, max_y);
    }

    let mut new_position = None;
    for (i, position) in positions.iter().enumerate() {
        if position.is_none() {
            // println!("next_pos- none");
            let mut settled = options[i].clone();
            settled.item = Item::Settle;
            settle_grid.push(settled);
            new_position = settle(&options[i].clone().fall(), grid, settle_grid, max_y);
            break;
        } else if let Some(next) = position {
            // println!("next_pos- {}", next);
            if next.is_wall() || next.is_sand() || next.is_start() {
                continue;
            } else {
                new_position = settle(&options[i].clone().fall(), grid, settle_grid, max_y);
                break;
            }
        }
    }
    if let Some(settled) = new_position.clone() {
        let mut settled_c = settled.clone();
        settled_c.item = Item::Settle;
        settle_grid.push(settled_c);
        // print_grid(settle_grid, true);
        // println!("Options: {:?}", positions);
        if settled == *sand {
            // panic!("ded");
        }
    }

    new_position
} //
