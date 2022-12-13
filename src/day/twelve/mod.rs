use std::{path, result};

pub fn run() {
    println!("Day 12");

    {
        println!("Part 1 - Sample");
        let result = part_1(include_str!("sample"));
        println!("  Result {}", result);
        println!("  Expected 31");
    }
}

fn print_grid(grid: &Grid) {
    println!(
        "{}",
        grid.iter()
            .map(|line| line
                .into_iter()
                .clone()
                .fold(String::new(), |acc, row| format!("{}{}", acc, row)))
            .fold(String::new(), |acc, row| format!("{acc}\n{row}"))
    );
    println!("");
}

enum Direction {
    Up,
    Down,
    Left,
    Right,
}
type Grid = Vec<Vec<char>>;
#[derive(Debug, PartialEq, Eq, Clone, Copy)]
struct Position {
    x: usize,
    y: usize,
}

impl Position {
    fn new(x: usize, y: usize) -> Self {
        Self { y, x }
    }
}

fn parse_input(input: &str) -> (Grid, Position, Position) {
    let mut grid: Vec<Vec<char>> = Vec::new();
    let mut start_pos = Position::new(0, 0);
    let mut end_pos = Position::new(0, 0);
    for (y, line) in input.lines().enumerate() {
        let mut line_vec: Vec<char> = Vec::new();

        for (x, position) in line.chars().enumerate() {
            if position == 'S' {
                start_pos = Position::new(x, y);
            } else if position == 'E' {
                end_pos = Position::new(x, y);
            }
            line_vec.push(position);
        }

        grid.push(line_vec);
    }
    (grid, start_pos, end_pos)
}

const SCORE: &str = "SabcdefghijklmnopqrstuvwxyzE";
fn can_move_to_next_elevation(start: &char, next: &char) -> bool {
    let start = if *start == 'S' { &'a' } else { start };
    let next = if *next == 'E' { &'z' } else { next };
    println!("{} {}", start, next);
    let diff: i32 = SCORE.chars().position(|ch| start == &ch).unwrap() as i32
        - SCORE.chars().position(|ch| next == &ch).unwrap() as i32;
    diff == 1 || diff == 0
}

fn move_direction(
    dir: Direction,
    start: &Position,
    path_grid: &mut Grid,
    grid: &Grid,
) -> (Option<Position>, Grid) {
    let mut new_path_grid = path_grid.clone();
    let curr_item = grid[start.y][start.x];
    match dir {
        Direction::Up => {
            let starty = if start.y + 1 <= grid.len() - 1 {
                start.y + 1
            } else {
                start.y
            };
            let next_path = path_grid[starty][start.x];
            let next_up = grid[starty][start.x];
            println!("next - up {}", next_up);
            if can_move_to_next_elevation(&curr_item, &next_up)
                && starty != start.y
                && next_path != '.'
            {
                new_path_grid[start.y + 1][start.x] = 'v';
                (
                    Some(Position::new(start.x, start.y + 1)),
                    new_path_grid.to_vec(),
                )
            } else {
                (None, path_grid.to_vec())
            }
        }
        Direction::Down => {
            let starty = if start.y as i32 - 1 > 0 {
                start.y - 1
            } else {
                start.y
            };
            let next_path = path_grid[starty][start.x];
            let next_up = grid[starty][start.x];
            println!("next - down {}", next_up);
            if can_move_to_next_elevation(&curr_item, &next_up)
                && starty != start.y
                && next_path != '.'
            {
                new_path_grid[start.y - 1][start.x] = '^';
                (
                    Some(Position::new(start.x, start.y - 1)),
                    new_path_grid.to_vec(),
                )
            } else {
                (None, path_grid.to_vec())
            }
        }
        Direction::Right => {
            let startx = if start.x + 1 <= grid[start.y].len() - 1 {
                start.x + 1
            } else {
                start.x
            };
            let next_path = path_grid[start.y][startx];
            let next_right = grid[start.y][startx];
            println!("next - right {}", next_right);
            if can_move_to_next_elevation(&curr_item, &next_right)
                && startx != start.x
                && next_path != '.'
            {
                new_path_grid[start.y][start.x + 1] = '>';
                (
                    Some(Position::new(start.x + 1, start.y)),
                    new_path_grid.to_vec(),
                )
            } else {
                (None, path_grid.to_vec())
            }
        }
        Direction::Left => {
            let startx = if start.x as i32 - 1 > 0 {
                start.x - 1
            } else {
                start.x
            };
            let next_path = path_grid[start.y][startx];
            let next_left = grid[start.y][startx];
            println!("next - left {}", next_left);
            if can_move_to_next_elevation(&curr_item, &next_left)
                && startx != start.x
                && next_path != '.'
            {
                new_path_grid[start.y][start.x - 1] = '<';
                (
                    Some(Position::new(start.x - 1, start.y)),
                    new_path_grid.to_vec(),
                )
            } else {
                (None, path_grid.to_vec())
            }
        }
    }
}

fn check_valid_moves(
    moves: &Vec<(Position, Grid)>,
    grid: &Grid,
    end: &Position,
) -> Vec<(Option<Position>, Grid)> {
    let mut new_moves: Vec<(Option<Position>, Grid)> = Vec::new();
    for (pos, path_grid) in moves {
        println!("start_position {:?}, end {:?}", pos, end);
        if pos == end {
            println!("start == end");
            new_moves.push((None, path_grid.clone()));
            break;
        } else {
            let mut new_path_grid = path_grid.clone();
            let valid_moves = get_valid_moves(pos, &mut new_path_grid, grid);
            if valid_moves.len() > 1 {
                let rec_new_moves = check_valid_moves(&valid_moves, grid, end);
                let position_of_end = rec_new_moves
                    .iter()
                    .position(|(pos, path_grid)| pos.is_none());
                new_moves.append((None, rec_new_moves[position_of_end]);
            } else {
                new_moves.append(valid_moves);
            }
        }
    }
    new_moves
}

fn get_valid_moves(start: &Position, path_grid: &mut Grid, grid: &Grid) -> Vec<(Position, Grid)> {
    let directions = Vec::from([
        move_direction(Direction::Right, &start, &mut path_grid, &grid),
        move_direction(Direction::Left, &start, &mut path_grid, &grid),
        move_direction(Direction::Up, &start, &mut path_grid, &grid),
        move_direction(Direction::Down, &start, &mut path_grid, &grid),
    ]);

    directions
        .into_iter()
        .filter(|(item, _)| item.is_some())
        .map(|(item, grid)| (item.unwrap(), grid))
        .collect::<Vec<(Position, Grid)>>()
}

// pos (x,y)
//find next moves
//perform move
//check if can move +/- to next position
//repeat
fn walk_grid(grid: &Grid, path_grid: &Grid, start: Position, end: Position) -> (bool, Grid) {
    println!("Walk");
    println!("start_position {:?}, end {:?}", start, end);

    let mut new_path_grid = path_grid.clone();
    let valid_directions = get_valid_moves(&start, &mut new_path_grid, grid);
    let valid_move = check_valid_moves(&valid_directions, grid, &end);
    println!("{:#?}", valid_move);
    (true, valid_move.1)
}

fn part_1(input: &str) -> i32 {
    let (grid, start, end) = parse_input(input);
    let empty_path: Vec<Vec<char>> = vec![vec!['.'; grid.len()]; grid.len()];
    let (finished, path_grid) = walk_grid(&grid, &empty_path, start, end);
    print_grid(&path_grid);
    print_grid(&grid);
    0
}
