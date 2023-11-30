use std::fmt::{self, Debug};
use std::time::{Duration, SystemTime};

#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub enum PositionItem {
    FallingRock,
    Wall,
    Floor,
    Corner,
    SettledRock,
}

#[derive(Clone, PartialEq, Eq, Hash)]
pub struct Position {
    pub x: usize,
    pub y: usize,
    pub item: PositionItem,
}

impl std::fmt::Debug for Position {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_fmt(format_args!("({},{})", self.x, self.y))
    }
}

impl Position {
    pub fn new(x: usize, y: usize, item: PositionItem) -> Self {
        Self { y, x, item }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum RockType {
    HLine,
    VLine,
    Cross,
    Square,
    BackwardsL,
}

#[derive(Clone, PartialEq, Eq)]
pub struct Rock {
    pub coords: Vec<Position>,
    pub rock_type: RockType,
}

impl fmt::Debug for Rock {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("Rock")
            .field("type", &self.rock_type)
            .field(
                "Coords",
                &self
                    .coords
                    .iter()
                    .map(|coord| format!("{:?}", coord))
                    .collect::<String>(),
            )
            .finish()
    }
}

impl Rock {
    pub fn new(r#type: RockType) -> Self {
        let mut coords = match r#type {
            RockType::HLine => vec![
                Position::new(0, 0, PositionItem::FallingRock),
                Position::new(1, 0, PositionItem::FallingRock),
                Position::new(2, 0, PositionItem::FallingRock),
                Position::new(3, 0, PositionItem::FallingRock),
            ],
            RockType::VLine => vec![
                Position::new(0, 0, PositionItem::FallingRock),
                Position::new(0, 1, PositionItem::FallingRock),
                Position::new(0, 2, PositionItem::FallingRock),
                Position::new(0, 3, PositionItem::FallingRock),
            ],
            RockType::Cross => vec![
                Position::new(1, 0, PositionItem::FallingRock),
                Position::new(0, 1, PositionItem::FallingRock),
                Position::new(1, 1, PositionItem::FallingRock),
                Position::new(2, 1, PositionItem::FallingRock),
                Position::new(1, 2, PositionItem::FallingRock),
            ],
            RockType::Square => vec![
                Position::new(0, 0, PositionItem::FallingRock),
                Position::new(0, 1, PositionItem::FallingRock),
                Position::new(1, 0, PositionItem::FallingRock),
                Position::new(1, 1, PositionItem::FallingRock),
            ],
            RockType::BackwardsL => vec![
                Position::new(0, 0, PositionItem::FallingRock),
                Position::new(1, 0, PositionItem::FallingRock),
                Position::new(2, 0, PositionItem::FallingRock),
                Position::new(2, 1, PositionItem::FallingRock),
                Position::new(2, 2, PositionItem::FallingRock),
            ],
        };
        coords.sort_by(|a, b| a.x.cmp(&b.x));
        Self {
            coords,
            rock_type: r#type,
        }
    }

    pub fn height_from_lower(&self, another: &Rock) -> usize {
        self.top_point().y - another.bottom_point().y - 1
    }

    pub fn move_up_by(&mut self, amount: usize) {
        self.coords.iter_mut().for_each(|coord| coord.y += amount);
    }

    pub fn same_x_and_type(&self, another: &Rock) -> bool {
        self.rock_type == another.rock_type
            && self
                .coords
                .iter()
                .enumerate()
                .all(|(i, coord)| coord.x == another.coords[i].x)
    }

    fn intersects_with_y(&self, another: &Rock, dir: isize) -> bool {
        self.coords.iter().rev().any(|coord| {
            let new_y = if dir > 0 {
                coord.y + (dir as usize)
            } else {
                coord.y - (dir * -1) as usize
            };
            another
                .coords
                .iter()
                .any(|cor| cor.x == coord.x && cor.y == new_y)
        })
    }

    fn intersects_with_x(&self, another: &Rock, dir: isize) -> bool {
        self.coords.iter().any(|coord| {
            let new_x = if dir > 0 {
                coord.x + (dir as usize)
            } else {
                coord.x - (dir * -1) as usize
            };
            another
                .coords
                .iter()
                .any(|cor| cor.x == new_x && cor.y == coord.y)
        })
    }

    fn collides_with_rocks_x(&self, other_rocks: &[Rock], dir: isize) -> bool {
        other_rocks
            .iter()
            .rev()
            .take(30)
            .any(|rock| self.intersects_with_x(rock, dir))
    }

    pub fn move_right_by(
        &mut self,
        amount: isize,
        width: usize,
        other_rocks: &Vec<Rock>,
        skip_check: bool,
    ) {
        if !skip_check && self.collides_with_rocks_x(&other_rocks, amount) {
            return;
        }

        let min_x = self.coords[0].x;
        let max_x = self.coords[self.coords.len() - 1].x;
        if max_x == width - 1 && amount > 0 || min_x == 1 && amount < 0 {
            return;
        }
        self.coords.iter_mut().for_each(|coord| {
            if amount > 0 {
                coord.x += amount as usize
            } else {
                coord.x -= (amount * -1) as usize
            }
        });
    }

    pub fn move_by_jet(
        &mut self,
        jet_stream: &mut JetStream,
        width: usize,
        rocks: &Vec<Rock>,
        skip_check: bool,
    ) {
        match jet_stream.get_next_loc() {
            '>' => self.move_right_by(1, width, rocks, skip_check),
            '<' => self.move_right_by(-1, width, rocks, skip_check),
            _ => panic!("unknown dir"),
        }
    }

    pub fn check_for_collision(&self, settled: &Vec<Rock>) -> bool {
        settled
            .iter()
            .rev()
            .take(30)
            .any(|rock| rock.intersects_with_y(self, 1))
    }

    pub fn settled(&mut self) {
        self.coords
            .iter_mut()
            .for_each(|coord| coord.item = PositionItem::SettledRock);
    }

    pub fn bottom_point(&self) -> &Position {
        self.coords.iter().min_by(|a, b| b.y.cmp(&a.y)).unwrap()
    }

    pub fn top_point(&self) -> &Position {
        self.coords.iter().max_by(|a, b| a.y.cmp(&b.y)).unwrap()
    }

    pub fn fall(&mut self) {
        self.coords.iter_mut().for_each(|coord| coord.y -= 1);
    }
}

#[derive(Debug)]
pub struct JetStream {
    stream: Vec<char>,
    loc: isize,
}

impl JetStream {
    pub fn new(jet_stream_chars: Vec<char>) -> Self {
        Self {
            stream: jet_stream_chars,
            loc: -1,
        }
    }

    pub fn get_next_loc(&mut self) -> char {
        let new_loc = if self.loc + 1 < self.stream.len() as isize {
            self.loc + 1
        } else {
            0
        };
        self.loc = new_loc;
        let index: usize = self.loc as usize;
        self.stream[index]
    }
}

#[derive(Debug)]
pub struct Chamber {
    width: usize,
    jet_stream: JetStream,
    count: usize,
    pub rocks: Vec<Rock>,
    rock_index: isize,
}

impl Chamber {
    pub fn new(width: usize, jet_stream_chars: Vec<char>, count: usize) -> Self {
        Self {
            width: width + 1,
            jet_stream: JetStream::new(jet_stream_chars),
            count,
            rocks: vec![
                Rock::new(RockType::HLine),
                Rock::new(RockType::Cross),
                Rock::new(RockType::BackwardsL),
                Rock::new(RockType::VLine),
                Rock::new(RockType::Square),
            ],
            rock_index: -1,
        }
    }

    pub fn get_next_rock(&mut self) -> Rock {
        let next_index = if self.rock_index + 1 < self.rocks.len() as isize {
            self.rock_index + 1
        } else {
            0
        };
        self.rock_index = next_index;
        let index: usize = self.rock_index.try_into().unwrap();
        self.rocks[index].clone()
    }

    fn get_bottom_point(&self, settled_rocks: &Vec<Rock>) -> usize {
        let bottom_y: usize = 1;
        let current_bottom = settled_rocks
            .iter()
            .rev()
            .take(5)
            .max_by(|rock_a, rock_b| rock_a.bottom_point().y.cmp(&rock_b.bottom_point().y));
        if let Some(bottom_rock) = current_bottom {
            bottom_rock.bottom_point().y + 1
        } else {
            bottom_y
        }
    }

    fn create_starting_positions(&self) -> Vec<Position> {
        let mut positions: Vec<Position> = Vec::new();
        for i in 0..=self.width {
            if i == 0 || i == self.width {
                positions.push(Position::new(i, 0, PositionItem::Corner));
            } else {
                positions.push(Position::new(i, 0, PositionItem::Floor));
            }
        }
        positions
    }

    fn fill_in_walls(&self, positions: &Vec<Position>, max_y: usize) -> Vec<Position> {
        let mut updated_positions = positions.clone();
        for i in 0..=max_y {
            let existing = updated_positions
                .iter()
                .find(|pos| pos.x == 0 && pos.y == i || pos.x == self.width && pos.y == i);
            if existing.is_none() {
                updated_positions.push(Position::new(0, i, PositionItem::Wall));
                updated_positions.push(Position::new(self.width, i, PositionItem::Wall));
            }
        }

        updated_positions
    }

    pub fn simulate(&mut self, animate: bool, perf_trace: bool, cycle_check: bool) -> usize {
        let mut positions = self.create_starting_positions();
        let mut settled_rocks: Vec<Rock> = Vec::new();

        let mut bottom_times: Vec<std::time::Duration> = Vec::new();
        let mut collision_times: Vec<std::time::Duration> = Vec::new();
        let mut jet_times: Vec<std::time::Duration> = Vec::new();

        let mut cycle_start = 0;
        let mut cycle_length = 0;

        let mut settled_count = 0;
        let mut cycle_check = cycle_check;
        loop {
            let mut next_rock = self.get_next_rock();

            let bottom_start = SystemTime::now();
            let next_bottom = self.get_bottom_point(&settled_rocks);
            let bottom_end = SystemTime::now();
            if perf_trace {
                bottom_times.push(
                    bottom_end
                        .duration_since(bottom_start)
                        .unwrap_or(Duration::from_secs(0)),
                )
            }

            next_rock.move_up_by(next_bottom + 3);
            next_rock.move_right_by(3, self.width, &settled_rocks, true);

            if animate {
                positions = self.fill_in_walls(&positions, next_bottom + 6);
                pretty_print_pos(&positions, Some(&next_rock), true);
            }

            next_rock.move_by_jet(&mut self.jet_stream, self.width, &settled_rocks, true);

            loop {
                let first_rock_at_bottom = settled_rocks.len() == 0
                    && next_rock
                        .coords
                        .iter()
                        .take(1)
                        .all(|pos| pos.y == next_bottom);

                let collision_start = SystemTime::now();
                let collision = next_rock.check_for_collision(&settled_rocks);
                let collision_end = SystemTime::now();
                collision_times.push(
                    collision_end
                        .duration_since(collision_start)
                        .unwrap_or(Duration::from_secs(0)),
                );

                if first_rock_at_bottom || collision {
                    next_rock.settled();

                    if animate {
                        positions.append(&mut next_rock.coords.clone());
                        pretty_print_pos(&positions, None, true);
                        pretty_print_rocks(&settled_rocks);
                    }

                    settled_rocks.push(next_rock);
                    settled_count += 1;
                    break;
                }

                next_rock.fall();

                let jet_start = SystemTime::now();
                next_rock.move_by_jet(&mut self.jet_stream, self.width, &settled_rocks, false);
                let jet_end = SystemTime::now();

                if perf_trace {
                    jet_times.push(
                        jet_end
                            .duration_since(jet_start)
                            .unwrap_or(Duration::from_secs(0)),
                    );
                }

                if animate {
                    pretty_print_pos(&positions, Some(&next_rock), true);
                    std::thread::sleep(std::time::Duration::from_millis(100));
                }
            }

            if cycle_check && settled_rocks.len() % 20 == 0 {
                let (cycle, start, length) =
                    check_for_cycle(&settled_rocks, cycle_start + cycle_length);
                if cycle && cycle_start == 0 {
                    println!("Found cycle");
                    cycle_start = start;
                    cycle_length = length;
                } else if cycle && length == cycle_length {
                    println!("Validated cycle (len {cycle_length}, start {cycle_start})");

                    let mut height_of_cycle = 0;
                    for i in 1..=2 {
                        let start_of_cycle = settled_rocks
                            .get(cycle_start + (cycle_length * (i - 1)))
                            .unwrap();
                        let end_of_cycle = settled_rocks
                            .get(cycle_start + (cycle_length * i) - 1)
                            .unwrap();
                        height_of_cycle = end_of_cycle.height_from_lower(start_of_cycle);
                        break;
                    }

                    // get count of cycles, copy first cycle, remove the rest
                    let full_cycles = (self.count - cycle_start) / cycle_length;
                    let mut pre_cycle = settled_rocks[0..cycle_start].to_vec();
                    let mut first_cycle =
                        settled_rocks[cycle_start..(cycle_start + cycle_length)].to_vec();
                    // println!("Start {}", 0);
                    // println!("Cycle Start {}", cycle_start);
                    // println!("Cycle End  {}", cycle_start + cycle_length - 1);

                    // update first cycle to y position of full_cycle count
                    first_cycle
                        .iter_mut()
                        .for_each(|rock| rock.move_up_by(height_of_cycle * (full_cycles - 1) - 2));
                    pre_cycle.append(&mut first_cycle);
                    // pre_cycle.append(&mut second);
                    settled_rocks = pre_cycle;

                    // update cycle count
                    cycle_check = false;
                    settled_count = cycle_start + full_cycles * cycle_length;
                    // run to end
                } else {
                    // cycle_start = settled_rocks.len() / 40;
                }
            }

            if settled_count == self.count {
                break;
            }
        }

        if perf_trace {
            println!(
                "collision {:?}",
                collision_times.iter().rev().take(10).sum::<Duration>() / 10
            );
            println!(
                "bottom {:?}",
                bottom_times.iter().rev().take(10).sum::<Duration>() / 10
            );
            println!(
                "jet {:?}",
                jet_times.iter().rev().take(10).sum::<Duration>() / 10
            );
        }

        settled_rocks.last().unwrap().top_point().y
    }
}

fn check_for_cycle(rocks: &Vec<Rock>, start: usize) -> (bool, usize, usize) {
    // println!("Checking for cycle ({})", rocks.len());
    // if rocks.len() > 320 {
    //     panic!();
    // }
    loop {
        let mut start = start;
        let result = 'inner: loop {
            //take 1, 2, 3rd... rocks
            if start > rocks.len() - 1 {
                break (false, 0, 0);
            }

            //find the next rock that's the same type and matching x
            //get its position
            let start_rock = rocks.get(start).unwrap();
            let next_rock = rocks.iter().enumerate().find(|(i, rock)| {
                if i > &start && rock.same_x_and_type(start_rock) {
                    true
                } else {
                    false
                }
            });

            if next_rock.is_none() {
                start += 1;
                continue;
            }

            let (next_matching_rock_index, next_matching_rock) = next_rock.unwrap();
            let start_rock_bottom = start_rock.bottom_point().y;
            let next_rock_bottom = next_matching_rock.bottom_point().y;

            // greater than 40 start looping
            if next_rock_bottom > start_rock_bottom && next_rock_bottom - start_rock_bottom >= 40 {
                // if 1 ,2 ,3 etc match build up an array
                let mut matches: Vec<&Rock> = Vec::new();
                let mut right_matches: Vec<&Rock> = Vec::new();
                let (left, right) = &rocks[start..].split_at(next_matching_rock_index - start);
                for (index, rock) in left.iter().enumerate() {
                    if matches.len() > 35 {
                        // println!("Matches");
                        let cycle = right_matches
                            .iter()
                            .enumerate()
                            .all(|(i, rock)| matches[i].same_x_and_type(rock));

                        // println!("Cycle = {}", cycle);
                        if cycle {
                            panic!();
                        }
                    }
                    if matches.len() > 0 {
                        let matches_end = rocks
                            .iter()
                            .position(|rock| rock == *matches.last().unwrap())
                            .unwrap();
                        let next_start = rocks
                            .iter()
                            .position(|rock| rock == *right_matches.first().unwrap())
                            .unwrap();
                        let cycle = right_matches
                            .iter()
                            .enumerate()
                            .all(|(i, rock)| matches[i].same_x_and_type(rock));

                        if matches_end + 1 == next_start - 1 && cycle {
                            let start_point = rocks
                                .iter()
                                .position(|rock| rock == *matches.first().unwrap())
                                .unwrap();
                            let end_point = rocks
                                .iter()
                                .position(|rock| rock == *matches.last().unwrap())
                                .unwrap();
                            break 'inner (cycle, start_point, end_point - start_point + 2);
                        }
                    }
                    if index < right.len() && rock.rock_type == right[index].rock_type {
                        matches.push(rock);
                        right_matches.push(&right[index]);
                    } else {
                        // check for cycle start
                        // break
                        if matches.len() > 0 {
                            matches.clear();
                            right_matches.clear();
                        }
                        break 'inner (false, 0, 0);
                    }
                }
            }

            // if no match take next rock and restart
            start += 1;
        };

        if start > rocks.len() - 1 {
            break (false, 0, 0);
        }
        break result;
    }
}

pub struct PrettyPositions(pub Vec<Position>);
impl fmt::Debug for PrettyPositions {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let items: Vec<&Position> = self.0.iter().collect();
        let max_x = items.iter().max_by(|x, y| x.x.cmp(&y.x)).unwrap().x;
        let min_x = items.iter().min_by(|x, y| x.x.cmp(&y.x)).unwrap().x;

        let max_y = items.iter().max_by(|x, y| x.y.cmp(&y.y)).unwrap().y;
        let mut min_y = items.iter().min_by(|x, y| x.y.cmp(&y.y)).unwrap().y;

        let visible_buffer: usize = 80;
        if max_y > visible_buffer {
            min_y += ((max_y as isize) - (visible_buffer as isize)) as usize;
        }

        for grid_y in (min_y..=max_y).rev() {
            if grid_y < 10 {
                print!(" {} ", grid_y);
            } else {
                print!("{} ", grid_y);
            }
            for grid_x in min_x..=max_x {
                let cord = items
                    .iter()
                    .position(|item| item.x == grid_x && item.y == grid_y);
                if let Some(cord) = cord {
                    match items[cord].item {
                        PositionItem::FallingRock => write!(f, "@")?,
                        PositionItem::SettledRock => write!(f, "#")?,
                        PositionItem::Floor => write!(f, "-")?,
                        PositionItem::Corner => write!(f, "+")?,
                        PositionItem::Wall => write!(f, "|")?,
                    }
                } else {
                    write!(f, ".")?;
                }
            }
            write!(f, "\n")?;
        }
        Ok(())
    }
}

fn pretty_print_rocks(rocks: &Vec<Rock>) {
    if rocks.len() > 0 {
        pretty_print_pos(
            &rocks
                .iter()
                .flat_map(|rock| rock.coords.clone())
                .collect::<Vec<Position>>(),
            None,
            false,
        );
    }
}

fn pretty_print_pos(positions: &Vec<Position>, next_rock: Option<&Rock>, clear: bool) {
    let next_pos = if let Some(rock) = next_rock {
        let mut pos = positions.clone();
        pos.append(&mut rock.coords.clone());
        pos
    } else {
        positions.clone()
    };
    if clear {
        print!("\x1B[2J\x1B[1;1H");
    }
    println!("{:?}", PrettyPositions(next_pos));
}
