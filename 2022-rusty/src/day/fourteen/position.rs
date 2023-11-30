#[derive(Clone, Copy, Debug, PartialEq)]
pub enum Item {
    Sand,
    Wall,
    Start,
    Settle,
}

#[derive(Clone, Debug, PartialEq)]
pub struct Position {
    pub x: isize,
    pub y: isize,
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
    pub fn new(x: isize, y: isize, item: Item) -> Self {
        Self { y, x, item }
    }

    pub fn fall(&mut self) -> Self {
        self.y += 1;
        self.to_owned()
    }

    pub fn is_start(&self) -> bool {
        self.item == Item::Start
    }

    pub fn is_wall(&self) -> bool {
        self.item == Item::Wall
    }

    pub fn is_sand(&self) -> bool {
        self.item == Item::Sand
    }

    pub fn get_points_between(&self, next: &Position) -> Vec<Position> {
        let mut points_between = Vec::new();
        let start = self;
        if self.x == next.x {
            // diff in y
            let diff = self.y - next.y;
            if diff > 0 {
                // down (inc-y)
                for _ in 0..diff {
                    let last: &Position = if points_between.len() > 0 {
                        points_between.last().unwrap()
                    } else {
                        start
                    };
                    points_between.push(Position::new(last.x, last.y - 1, last.item));
                }
            } else {
                // up (dec-y)
                for _ in 0..(diff * -1) {
                    let last: &Position = if points_between.len() > 0 {
                        points_between.last().unwrap()
                    } else {
                        start
                    };
                    points_between.push(Position::new(last.x, last.y + 1, last.item));
                }
            }
        } else if self.y == next.y {
            // moving in x
            let diff = self.x - next.x;
            if diff > 0 {
                // left (inc-x)
                for _ in 0..diff {
                    let last: &Position = if points_between.len() > 0 {
                        points_between.last().unwrap()
                    } else {
                        start
                    };
                    points_between.push(Position::new(last.x - 1, last.y, last.item));
                }
            } else {
                // right (dec-x)
                for _ in 0..(diff * -1) {
                    let last: &Position = if points_between.len() > 0 {
                        points_between.last().unwrap()
                    } else {
                        start
                    };
                    points_between.push(Position::new(last.x + 1, last.y, last.item));
                }
            }
        }
        points_between
    }
}
