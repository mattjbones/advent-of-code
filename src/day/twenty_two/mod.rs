use crate::Runner;
use regex::Regex;

mod util;
use util::*;

pub struct TwentyTwo {}
impl Runner<usize> for TwentyTwo {
    fn part_1_sample(&self, sample: &str) -> (usize, usize) {
        let (board, instructions) = parse_input(sample);

        println!("{:#?}", instructions);

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
    instructions: &Vec<(Option<String>, usize)>,
) -> Vec<Position> {
    let mut working_board = board.clone();
    let start_pos = working_board
        .iter_mut()
        .find(|pos| pos.item == Item::Open)
        .unwrap();
    start_pos.item = Item::Right;

    // instructions
    //     .iter()
    //     .for_each(|instruction| instruction.split_inclusive(pat));

    working_board
}

fn parse_input(input: &str) -> (Vec<Position>, Vec<(Option<String>, usize)>) {
    let board_break = input.lines().position(|line| line == "").unwrap();
    let lines = input.lines().collect::<Vec<&str>>();
    let (board, instructions) = lines.split_at(board_break);
    let board_pos = build_board_from_input(board);
    let instructions = parse_instructions(instructions);
    (board_pos, instructions)
}

fn parse_instructions(input: &[&str]) -> Vec<(Option<String>, usize)> {
    let re = Regex::new(r"^(?P<start>\d*)|(?P<dir>[RL])(?P<amount>\d*)").unwrap();
    re.captures_iter(input.last().unwrap())
        .map(|res| {
            println!("{:?}", res);

            let start = res
                .name("start")
                .map_or(None, |start| start.as_str().parse::<usize>().ok());

            let dir = res
                .name("dir")
                .map_or(None, |dir| Some(dir.as_str().to_string()));

            let amount = res
                .name("amount")
                .map_or(None, |amount| amount.as_str().parse::<usize>().ok());

            let count = start.unwrap_or(amount.unwrap());

            println!("{:?} {:?} {:?} ({})", start, dir, amount, count);
            (dir, count)
        })
        .collect::<Vec<(Option<String>, usize)>>()
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
