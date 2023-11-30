use crate::Runner;
use regex::Regex;

mod util;
use util::*;

pub struct TwentyTwo {}
impl Runner<usize> for TwentyTwo {
    fn part_1_sample(&self, sample: &str) -> (usize, usize) {
        let (board, instructions) = parse_input(sample);
        pretty_print_pos(&board, false);

        // println!("{:#?}", instructions);

        let final_board = execute_instructions(&board, &instructions);
        pretty_print_pos(&final_board, false);

        let result = 1;
        (result, 6032)
    }

    fn part_1_input(&self, _: &str) -> (usize, usize) {
        let result = 1;
        (result, 0)
    }

    fn part_2_sample(&self, _: &str) -> (usize, usize) {
        let result = 1;
        (result, 0)
    }

    fn part_2_input(&self, _: &str) -> (usize, usize) {
        let result = 1;
        (result, 0)
    }
}

fn execute_instructions(
    board: &Vec<Position>,
    instructions: &Vec<(Option<Direction>, usize)>,
) -> Vec<Position> {
    let mut working_board = board.clone();
    working_board
        .iter_mut()
        .find(|pos| pos.item == Item::Open)
        .unwrap()
        .set_dir(Some(90))
        .set_item(Item::Head);

    instructions.iter().for_each(|(dir, count)| {
        let mut current_pos = working_board
            .iter()
            .find(|pos| pos.item == Item::Head)
            .unwrap()
            .clone();
        current_pos.change_direction(dir);

        println!("Next direction {dir:?} {:?}", current_pos.dir);
        println!("Count {}", count);
        let mut next_positions = match current_pos.dir.unwrap() {
            90 => {
                // increase x
                working_board
                    .iter_mut()
                    .filter(|el| el.y == current_pos.y)
                    .collect::<Vec<&mut Position>>()
            }
            180 => {
                // increase y
                working_board
                    .iter_mut()
                    .filter(|el| el.y == current_pos.y)
                    .collect::<Vec<&mut Position>>()
            }
            270 => {
                // decrease x
                working_board
                    .iter_mut()
                    .rev()
                    .filter(|el| el.x == current_pos.x)
                    .collect::<Vec<&mut Position>>()
            }
            360 => {
                // decrease y
                working_board
                    .iter_mut()
                    .rev()
                    .filter(|el| el.y == current_pos.y)
                    .collect::<Vec<&mut Position>>()
            }
            _ => panic!("unknown dir"),
        };

        let start_index = &next_positions
            .iter()
            .position(|pos| pos.x == current_pos.x && pos.y == current_pos.y)
            .unwrap();

        pretty_print_mut_pos(&next_positions, false);
        let next_wall = &next_positions.iter().position(|pos| pos.item == Item::Wall);

        if let Some(next_wall) = next_wall {
            let (_, to_fill) = next_positions.split_at_mut(*start_index);
            if count >= next_wall {
                // fill everything to the wall
                let to_fill_without_wall = to_fill.split_at_mut(*next_wall).0;

                to_fill_without_wall.iter_mut().for_each(|pos| {
                    if pos.item != Item::Wall {
                        pos.set_item(Item::Open);
                        pos.set_dir(current_pos.dir);
                    }
                });

                to_fill_without_wall
                    .last_mut()
                    .unwrap()
                    .set_dir(current_pos.dir)
                    .set_item(Item::Head);

                pretty_print_pos(&working_board, false);
            } else {
                //fill everything to the count
                let mut filling = to_fill.iter_mut();
                for _ in 0..*count {
                    //check next position
                    filling
                        .next()
                        .unwrap()
                        .set_item(Item::Open)
                        .set_dir(current_pos.dir);
                }
            }
        } else {
            //handle wrapping
            panic!("ded");
        }
    });

    working_board
}

fn parse_input(input: &str) -> (Vec<Position>, Vec<(Option<Direction>, usize)>) {
    let board_break = input.lines().position(|line| line == "").unwrap();
    let lines = input.lines().collect::<Vec<&str>>();
    let (board, instructions) = lines.split_at(board_break);
    let board_pos = build_board_from_input(board);
    let instructions = parse_instructions(instructions);
    (board_pos, instructions)
}

fn parse_instructions(input: &[&str]) -> Vec<(Option<Direction>, usize)> {
    let re = Regex::new(r"^(?P<start>\d*)|(?P<dir>[RL])(?P<amount>\d*)").unwrap();
    re.captures_iter(input.last().unwrap())
        .map(|res| {
            // println!("{:?}", res);

            let start = res
                .name("start")
                .map_or(None, |start| start.as_str().parse::<usize>().ok());

            let dir = res.name("dir").map_or(None, |dir| match dir.as_str() {
                "L" => Some(Direction::Left),
                "R" => Some(Direction::Right),
                _ => panic!("unknown"),
            });

            let amount = res
                .name("amount")
                .map_or(None, |amount| amount.as_str().parse::<usize>().ok());

            let count = if let Some(start) = start {
                start
            } else {
                amount.unwrap()
            };
            (dir, count)
        })
        .collect::<Vec<(Option<Direction>, usize)>>()
}

fn build_board_from_input(input: &[&str]) -> Vec<Position> {
    input
        .iter()
        .enumerate()
        .flat_map(|(row, line)| {
            line.chars()
                .enumerate()
                .filter_map(|(col, char)| {
                    if char != ' ' {
                        Some(Position::new(col, row, char))
                    } else {
                        None
                    }
                })
                .collect::<Vec<Position>>()
        })
        .collect()
}

#[cfg(test)]
mod tests {}
