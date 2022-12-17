use std::fmt;

#[derive(Copy, Clone, Debug, PartialEq, Hash, Eq)]
pub struct Position {
    pub x: usize,
    pub y: usize,
}

impl std::fmt::Display for Position {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_fmt(format_args!("({},{}", self.x, self.y))
    }
}

impl Position {
    pub fn new(x: usize, y: usize) -> Self {
        Self { y, x }
    }
}

#[derive(Debug)]
pub struct Chamber {
    width: usize,
    jet_stream: Vec<char>,
    count: usize,
    pub rocks: Vec<Rock>,
}

#[derive(Debug, Clone, Copy)]
pub enum RockType {
    HLine,
    VLine,
    Cross,
    Square,
    BackwardsL,
}

#[derive(Debug)]
pub struct Rock {
    pub coords: Vec<Position>,
    pub rock_type: RockType,
}

impl Rock {
    pub fn new(r#type: RockType) -> Self {
        let coords = match r#type {
            RockType::HLine => vec![
                Position::new(0, 0),
                Position::new(1, 0),
                Position::new(2, 0),
                Position::new(3, 0),
            ],
            RockType::VLine => vec![
                Position::new(0, 0),
                Position::new(0, 1),
                Position::new(0, 2),
                Position::new(0, 3),
            ],
            RockType::Cross => vec![
                Position::new(1, 0),
                Position::new(0, 1),
                Position::new(1, 1),
                Position::new(2, 1),
                Position::new(1, 2),
            ],
            RockType::Square => vec![
                Position::new(0, 0),
                Position::new(0, 1),
                Position::new(1, 0),
                Position::new(1, 1),
            ],
            RockType::BackwardsL => vec![
                Position::new(2, 0),
                Position::new(2, 1),
                Position::new(0, 2),
                Position::new(1, 2),
                Position::new(2, 2),
            ],
        };

        Self {
            coords,
            rock_type: r#type,
        }
    }
}

impl Chamber {
    pub fn new(width: usize, jet_stream: Vec<char>, count: usize) -> Self {
        Self {
            width,
            jet_stream,
            count,
            rocks: vec![
                Rock::new(RockType::HLine),
                Rock::new(RockType::Cross),
                Rock::new(RockType::BackwardsL),
                Rock::new(RockType::VLine),
                Rock::new(RockType::Square),
            ],
        }
    }

    pub fn simulate(&self) -> usize {
        let positions: Vec<Position> = Vec::new();
        let mut settled_rocks: Vec<Position> = Vec::new();
        let mut jet_loc = 0;
        loop {
            let new_rock = self.rocks[rock_index].clone();
            let current_bottom = settled_rocks
                .iter()
                .max_by(|rockA, rockB| rockA.y.cmp(&rockB.y))
                .unwrap_or(Position::new(0, 0));
            new_rock.move_up_by(current_bottom + 3);
            new_rock.move_right_by(2);

            loop {
                if !new_rock.check_for_collisions(settled_rocks) {
                    new_rock.push(self.jet_stream, jet_loc);
                    jet_loc = if jet_loc < self.jet_stream.len() {
                        jet_loc + 1
                    } else {
                        0
                    };
                    new_rock.fall();
                } else {
                    settled_rocks.push(new_rock);
                    break;
                }
            }

            if settled_rocks.len() == self.count {
                break;
            }
        }
        positions.len()
    }
}

pub struct PrettyPositions(pub Box<Vec<Position>>, pub usize);
impl fmt::Debug for PrettyPositions {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let max = self.1;
        let items: Vec<&Position> = self.0.iter().collect();
        let max_x = items.iter().max_by(|x, y| x.x.cmp(&y.x)).unwrap().x + 1;
        let min_x = items.iter().min_by(|x, y| x.x.cmp(&y.x)).unwrap().x;

        let max_y = items.iter().max_by(|x, y| x.y.cmp(&y.y)).unwrap().y + 1;
        let min_y = items.iter().min_by(|x, y| x.y.cmp(&y.y)).unwrap().y;

        for grid_y in min_y..max_y {
            for grid_x in min_x..max_x {
                let cord = items
                    .iter()
                    .rev()
                    .position(|item| item.x == grid_x && item.y == grid_y);
                if cord.is_some() {
                    write!(f, "#")?;
                } else {
                    write!(f, ".")?;
                }
            }
            write!(f, "\n")?;
        }
        Ok(())
    }
}
