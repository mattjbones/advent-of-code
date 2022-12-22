use std::fmt;

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

        for grid_y in min_y..=max_y {
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
                        Item::Wall => write!(f, "#")?,
                        Item::Open => write!(f, ".")?,
                        Item::Up => write!(f, "^")?,
                        Item::Left => write!(f, "<")?,
                        Item::Right => write!(f, ">")?,
                        Item::Down => write!(f, "v")?,
                    }
                } else {
                    write!(f, " ")?;
                }
            }
            write!(f, "\n")?;
        }
        Ok(())
    }
}

pub fn pretty_print_pos(positions: &Vec<Position>, clear: bool) {
    if clear {
        print!("\x1B[2J\x1B[1;1H");
    }
    println!("{:?}", PrettyPositions(positions.clone()));
}

#[derive(Clone, Debug, PartialEq, Hash, Eq)]
pub enum Item {
    Wall,
    Open,
    Left,
    Right,
    Up,
    Down,
}

#[derive(Clone, Debug, PartialEq, Hash, Eq)]
pub struct Position {
    pub x: usize,
    pub y: usize,
    pub item: Item,
}

impl std::fmt::Display for Position {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_fmt(format_args!(
            "({},{}, type={:?})",
            self.x, self.y, self.item
        ))
    }
}

impl Position {
    pub fn new(x: usize, y: usize, item: char) -> Self {
        let item = match item {
            '.' => Item::Open,
            '#' => Item::Wall,
            _ => panic!("Unknown input"),
        };
        Self { y, x, item }
    }
}
