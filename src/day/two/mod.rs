use std::env;
use std::fs::File;
use std::io;
use std::io::BufRead;

const INPUT: &str = "/src/day/two/input";

const SCORE_ROCK: usize = 1;
const SCORE_PAPER: usize = 2;
const SCORE_SCISSORS: usize = 3;

const SCORE_WIN: usize = 6;
const SCORE_DRAW: usize = 3;
const SCORE_LOSS: usize = 0;

#[derive(Debug)]
enum Play {
    Rock,
    Paper,
    Scissors,
}

#[derive(Debug)]
enum RoundResult {
    Win(usize),
    Draw(usize),
    Loss(usize),
}

impl RoundResult {
    fn unwrap(&self) -> usize {
        match self {
            RoundResult::Draw(result) => *result,
            RoundResult::Win(result) => *result,
            RoundResult::Loss(result) => *result,
        }
    }

    fn new(m: &str) -> Self {
        match m {
            "X" => RoundResult::Loss(SCORE_LOSS),
            "Y" => RoundResult::Draw(SCORE_DRAW),
            "Z" => RoundResult::Win(SCORE_WIN),
            &_ => panic!("Unknown result"),
        }
    }
}

impl Play {
    fn round_result(&self, challenger: &Play) -> RoundResult {
        match self {
            Play::Rock => match challenger {
                Play::Rock => RoundResult::Draw(SCORE_DRAW + SCORE_ROCK),
                Play::Paper => RoundResult::Loss(SCORE_LOSS + SCORE_ROCK),
                Play::Scissors => RoundResult::Win(SCORE_WIN + SCORE_ROCK),
            },
            Play::Paper => match challenger {
                Play::Rock => RoundResult::Win(SCORE_WIN + SCORE_PAPER),
                Play::Paper => RoundResult::Draw(SCORE_DRAW + SCORE_PAPER),
                Play::Scissors => RoundResult::Loss(SCORE_LOSS + SCORE_PAPER),
            },
            Play::Scissors => match challenger {
                Play::Rock => RoundResult::Loss(SCORE_LOSS + SCORE_SCISSORS),
                Play::Paper => RoundResult::Win(SCORE_WIN + SCORE_SCISSORS),
                Play::Scissors => RoundResult::Draw(SCORE_DRAW + SCORE_SCISSORS),
            },
        }
    }

    fn target_move(&self, round_result: &RoundResult) -> Play {
        match round_result {
            RoundResult::Draw(_) => match self {
                Play::Rock => Play::Rock,
                Play::Paper => Play::Paper,
                Play::Scissors => Play::Scissors,
            },
            RoundResult::Win(_) => match self {
                Play::Rock => Play::Paper,
                Play::Paper => Play::Scissors,
                Play::Scissors => Play::Rock,
            },
            RoundResult::Loss(_) => match self {
                Play::Rock => Play::Scissors,
                Play::Paper => Play::Rock,
                Play::Scissors => Play::Paper,
            },
        }
    }

    fn new(m: &str) -> Self {
        match m {
            "A" => Play::Rock,
            "B" => Play::Paper,
            "C" => Play::Scissors,
            "X" => Play::Rock,
            "Y" => Play::Paper,
            "Z" => Play::Scissors,
            &_ => panic!("Unknown move"),
        }
    }
}

pub fn run() {
    println!("Day two");
    let path = env::current_dir().unwrap();

    {
        println!("Part 1");
        let file = File::open(format!("{}{}", path.display(), INPUT)).unwrap();
        let lines = io::BufReader::new(&file).lines();

        let mut total = 0;
        for line in lines {
            if let Ok(line) = line {
                let moves: Vec<Play> = line.split(" ").map(|m| Play::new(m)).collect();
                total += moves[1].round_result(&moves[0]).unwrap();
            }
        }
        println!("Total: {}", total);
    }

    {
        println!("Part 2");
        let file = File::open(format!("{}{}", path.display(), INPUT)).unwrap();
        let lines = io::BufReader::new(file).lines();
        let mut total = 0;
        for line in lines {
            if let Ok(line) = line {
                let mut split = line.split(" ");
                let challenger_move = Play::new(split.next().unwrap());
                let target_result = RoundResult::new(split.next().unwrap());
                let player_move = challenger_move.target_move(&target_result);
                total += player_move.round_result(&challenger_move).unwrap();
            }
        }
        println!("Total: {}", total);
    }
}
