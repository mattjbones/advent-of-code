use std::fmt;
use std::io::{Error, ErrorKind};

#[derive(Debug, PartialEq, Clone)]
struct Position {
    row: usize,
    col: usize,
}

impl Position {
    fn new(row: usize, col: usize) -> Self {
        Self { row, col }
    }

    fn is_start_position(&self, start_position: &Position) -> bool {
        if self == start_position {
            true
        } else {
            false
        }
    }

    fn distance_between_points(&self, another: &Position) -> usize {
        (((self.row as i8 - another.row as i8).pow(2) + (another.col as i8 - self.col as i8).pow(2))
            as f32)
            .sqrt() as usize
    }
}

#[derive(Debug, Clone)]
enum PositionItem {
    Start,
    Head,
    Tail,
    Both,
    Neither,
    Visited,
}

impl TryFrom<char> for PositionItem {
    type Error = std::io::Error;

    fn try_from(value: char) -> Result<Self, Self::Error> {
        let value_string = value.to_string();
        let value_str = value_string.as_str();
        match value_str {
            "." => Ok(PositionItem::Neither),
            "s" => Ok(PositionItem::Start),
            "H" => Ok(PositionItem::Head),
            "T" => Ok(PositionItem::Tail),
            val => {
                println!("unknown {val}");
                Err(Error::from(ErrorKind::Unsupported))
            }
        }
    }
}

struct PrettyGrid<'a>(&'a Grid);
fn print_grid(grid: &Grid) {
    println!("{:?}", PrettyGrid(grid));
}

impl fmt::Debug for PrettyGrid<'_> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        for row in self.0.iter() {
            let mut both_row = false;
            for pos in row {
                let pos = match pos {
                    PositionItem::Both => {
                        both_row = true;
                        "H"
                    }
                    PositionItem::Head => "H",
                    PositionItem::Neither => ".",
                    PositionItem::Tail => "T",
                    PositionItem::Start => "s",
                    PositionItem::Visited => "#",
                };
                write!(f, "{pos}")?;
            }
            if both_row {
                write!(f, " (Both)")?;
            }
            write!(f, "\n")?;
        }

        Ok(())
    }
}

impl PositionItem {
    fn is_head(&self) -> bool {
        match self {
            PositionItem::Head => true,
            PositionItem::Both => true,
            _ => false,
        }
    }

    fn is_visited(&self) -> bool {
        match self {
            PositionItem::Visited => true,
            _ => false,
        }
    }
}

type Grid = Vec<Vec<PositionItem>>;
type Instructions = Vec<(String, usize)>;

fn build_grid_from_input_string(input: &str) -> Result<Grid, Error> {
    let mut grid: Grid = Vec::new();
    for (i, line) in input.lines().enumerate() {
        if line == "" {
            break;
        }

        if grid.get(i).is_none() {
            grid.push(Vec::new());
        }

        for (char_index, char) in line.chars().enumerate() {
            if char == ' ' {
                let meta = line
                    .split_at(char_index + 2)
                    .1
                    .replace("(", "")
                    .replace(")", "")
                    .replace(",", "");
                let parts: Vec<&str> = meta.split(" ").collect();
                if parts[0] != "H" {
                    break;
                }

                //as Head moves we don't care about anything else
                let head_pos = grid[i].iter().position(|item| item.is_head()).unwrap();
                grid[i][head_pos] = PositionItem::Both;
                break;
            } else {
                let position = PositionItem::try_from(char)?;
                grid[i].push(position);
            }
        }
    }
    Ok(grid)
}

fn build_grid_from_max_input(length: usize) -> Grid {
    let mut grid = Vec::new();
    for _ in 0..length + 1 {
        let mut row = Vec::new();
        for _ in 0..length + 1 {
            row.push(PositionItem::Neither);
        }
        grid.push(row)
    }
    grid
}

fn build_instructions_from_input_string(input: &str) -> Instructions {
    input
        .lines()
        .into_iter()
        .filter(|line| {
            line.contains("R") || line.contains("L") || line.contains("U") || line.contains("D")
        })
        .map(|line| {
            let parts: Vec<&str> = line.split(" ").collect();
            (parts[0].to_string(), parts[1].parse::<usize>().unwrap())
        })
        .collect()
}

fn get_updated_items(
    item: &PositionItem,
    current: &PositionItem,
    is_start_position: bool,
    skip_tail_trace: bool,
) -> (PositionItem, PositionItem) {
    let new_item = match item {
        //we move head out of tail
        PositionItem::Both => PositionItem::Head,
        PositionItem::Head => {
            if current.is_head() {
                PositionItem::Both
            } else {
                PositionItem::Head
            }
        }
        item => item.clone(),
    };

    let after_move = match item {
        PositionItem::Head => PositionItem::Neither,
        PositionItem::Both => PositionItem::Tail,
        PositionItem::Tail => {
            if is_start_position {
                PositionItem::Start
            } else {
                PositionItem::Neither
            }
        }
        item => panic!("don't care {item:?}"),
    };

    (new_item, after_move)
}

fn move_in_column(
    grid: &mut Grid,
    item_position: &Position,
    is_start_position: bool,
    inc: bool,
    skip_tail_trace: bool,
) -> Position {
    // if !inc && item_position.row as i8 - 1 < 0
    //     || inc && item_position.row as i8 + 1 > (grid.len()) as i8
    // {
    //     println!("Move in col");
    //     println!("{item_position:?} {inc}");
    //     panic!("cannot move out of grid");
    // }

    let move_amount: isize = if inc { 1 } else { -1 };
    let new_pos: usize = (item_position.row as isize + move_amount) as usize;

    let item = &grid[item_position.row][item_position.col];
    let target_position_item = &grid[new_pos][item_position.col];
    // let (new_item, after_move) = get_updated_items(
    //     item,
    //     target_position_item,
    //     is_start_position,
    //     skip_tail_trace,
    // );
    // grid[new_pos][item_position.col] = new_item;
    // grid[item_position.row][item_position.col] = after_move;
    Position::new(new_pos, item_position.col)
}

fn move_in_row(
    grid: &mut Grid,
    item_position: &Position,
    is_start_position: bool,
    inc: bool,
    skip_tail_trace: bool,
) -> Position {
    // if inc && item_position.col + 1 > grid[item_position.row].len()
    //     || !inc && item_position.col as i8 - 1 < 0
    // {
    //     println!("Move in row");
    //     println!("{item_position:?} {inc}");
    //     panic!("cannot move out of grid");
    // }

    let move_amount: isize = if inc { 1 } else { -1 };
    let new_pos: usize = (item_position.col as isize + move_amount) as usize;

    let item = &grid[item_position.row][item_position.col];
    let target_position_item = &grid[item_position.row][new_pos];
    // let (new_item, after_move) = get_updated_items(
    //     item,
    //     target_position_item,
    //     is_start_position,
    //     skip_tail_trace,
    // );
    // grid[item_position.row][new_pos] = new_item;
    // grid[item_position.row][item_position.col] = after_move;
    Position::new(item_position.row, new_pos)
}

fn apply_instructions_to_grid(
    grid: &mut Grid,
    instructions: &Instructions,
    start_position: Position,
) {
    let mut head_position: Position = Position::new(grid.len() - 1, 0);
    let mut tail_position: Position = Position::new(grid.len() - 1, 0);
    let mut tail_positions = Vec::new();
    for (dir, times) in instructions {
        // println!("== {dir} {times} ==\n");
        for _ in 0..*times {
            //move H in dir
            let is_head_at_start = head_position.is_start_position(&start_position);
            let new_head_position = match dir.as_str() {
                "U" => move_in_column(grid, &head_position, is_head_at_start, false, false),
                "D" => move_in_column(grid, &head_position, is_head_at_start, true, false),
                "L" => move_in_row(grid, &head_position, is_head_at_start, false, false),
                "R" => move_in_row(grid, &head_position, is_head_at_start, true, false),
                _ => panic!("unknown"),
            };
            head_position = new_head_position;

            // check if we need move tail
            let head_tail_diff = head_position.distance_between_points(&tail_position);
            if head_tail_diff > 1 {
                let is_tail_at_start = tail_position.is_start_position(&start_position);
                let new_tail_position = match dir.as_str() {
                    "U" => {
                        if tail_position.col != head_position.col {
                            let sub_tail_move = move_in_row(
                                grid,
                                &tail_position,
                                is_tail_at_start,
                                (head_position.col as i8 - tail_position.col as i8) > 0,
                                false,
                            );
                            move_in_column(grid, &sub_tail_move, is_tail_at_start, false, true)
                        } else {
                            move_in_column(grid, &tail_position, is_tail_at_start, false, false)
                        }
                    }
                    "D" => {
                        if tail_position.col != head_position.col {
                            let sub_tail_move = move_in_row(
                                grid,
                                &tail_position,
                                is_tail_at_start,
                                (head_position.col as i8 - tail_position.col as i8) > 0,
                                false,
                            );
                            move_in_column(grid, &sub_tail_move, is_tail_at_start, true, true)
                        } else {
                            move_in_column(grid, &tail_position, is_tail_at_start, true, false)
                        }
                    }
                    "L" => {
                        if tail_position.row != head_position.row {
                            let sub_tail_move = move_in_column(
                                grid,
                                &tail_position,
                                is_tail_at_start,
                                (head_position.row as i8 - tail_position.row as i8) > 0,
                                false,
                            );
                            move_in_row(grid, &sub_tail_move, is_tail_at_start, false, true)
                        } else {
                            move_in_row(grid, &tail_position, is_tail_at_start, false, false)
                        }
                    }
                    "R" => {
                        if tail_position.row != head_position.row {
                            let sub_tail_move = move_in_column(
                                grid,
                                &tail_position,
                                is_tail_at_start,
                                (head_position.row as i8 - tail_position.row as i8) > 0,
                                false,
                            );
                            move_in_row(grid, &sub_tail_move, is_tail_at_start, true, true)
                        } else {
                            move_in_row(grid, &tail_position, is_tail_at_start, true, false)
                        }
                    }
                    _ => panic!("unknown"),
                };
                tail_positions.push(new_tail_position.clone());
                tail_position = new_tail_position;
            }

            //print grid
            // print_grid(grid);
        }
    }

    println!("Tail positions {}", tail_positions.len());
}

fn part_1(input_string: &str) -> Result<(), Error> {
    // let grid = &mut build_grid_from_input_string(input_string)?;
    let instructions = build_instructions_from_input_string(input_string);
    let max = instructions.iter().max().unwrap().1 * 100;
    let grid = &mut build_grid_from_max_input(max);
    let start_position = Position::new(grid.len() - 1, 0);
    grid[start_position.row][start_position.col] = PositionItem::Both;
    // print_grid(&grid);
    apply_instructions_to_grid(grid, &instructions, start_position);
    Ok(())
}

pub fn run() -> Result<(), Error> {
    println!("Day 9");

    {
        let input_string = include_str!("sample");
        println!("Part 1 - sample");
        part_1(input_string)?;
    }

    {
        let input_string = include_str!("input");
        println!("Part 1 - input");
        part_1(input_string)?;
    }

    Ok(())
}
