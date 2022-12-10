use std::collections::HashSet;
use std::fmt;

type Instructions = Vec<(String, usize)>;

#[derive(Clone, Debug, PartialEq, Eq, Hash)]
struct Location {
    x: i32,
    y: i32,
}

impl Location {
    fn new(x: i32, y: i32) -> Self {
        Self { x, y }
    }

    fn from((x, y): (i32, i32)) -> Self {
        Self { x, y }
    }
}

struct PrettyBody<'a>(&'a HashSet<Location>);
impl fmt::Debug for PrettyBody<'_> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let items: Vec<&Location> = self.0.iter().collect();
        let max_x = items.iter().max_by(|x, y| x.x.cmp(&y.x)).unwrap().x + 2;
        let min_x = items.iter().min_by(|x, y| x.x.cmp(&y.x)).unwrap().x - 1;

        let max_y = items.iter().max_by(|x, y| x.y.cmp(&y.y)).unwrap().y + 2;
        let min_y = items.iter().min_by(|x, y| x.y.cmp(&y.y)).unwrap().y - 1;

        for grid_y in (min_y..max_y).rev() {
            for grid_x in min_x..max_x {
                let cord = items
                    .iter()
                    .rev()
                    .position(|item| item.x == grid_x && item.y == grid_y);
                if grid_x == 0 && grid_y == 0 {
                    write!(f, "s")?;
                } else if cord.is_some() {
                    write!(f, "x")?;
                } else {
                    write!(f, ".")?;
                }
            }
            write!(f, "\n")?;
        }

        Ok(())
    }
}

const BUFFER: i32 = 3;
struct PrettyTail<'a>(&'a Vec<Location>);
impl fmt::Debug for PrettyTail<'_> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let max_x = self.0.iter().max_by(|x, y| x.x.cmp(&y.x)).unwrap().x + BUFFER + 1;
        let max_y = self.0.iter().max_by(|x, y| x.y.cmp(&y.y)).unwrap().y + BUFFER + 1;

        let min_x = self.0.iter().min_by(|x, y| x.x.cmp(&y.x)).unwrap().x - BUFFER;
        let min_y = self.0.iter().min_by(|x, y| x.y.cmp(&y.y)).unwrap().y - BUFFER;

        for grid_y in (min_y..max_y).rev() {
            for grid_x in min_x..max_x {
                let cord = self
                    .0
                    .iter()
                    .position(|item| item.x == grid_x && item.y == grid_y);

                if let Some(cord) = cord {
                    if cord == self.0.len() {
                        write!(f, "T")?;
                    } else if cord == 0 {
                        write!(f, "H")?;
                    } else {
                        write!(f, "{}", cord)?;
                    }
                } else if grid_x == 0 && grid_y == 0 {
                    write!(f, "s")?;
                } else {
                    write!(f, ".")?;
                }
            }
            write!(f, "\n")?;
        }

        Ok(())
    }
}

fn distance_between_points(first: &Location, second: &Location) -> f32 {
    (((first.x - second.x).pow(2) + (first.y - second.y).pow(2)) as f32).sqrt() as f32
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

fn move_horizontal(item: &Location, inc: bool) -> Location {
    if inc {
        Location::from((item.x + 1, item.y))
    } else {
        Location::from((item.x - 1, item.y))
    }
}

fn move_vertical(item: &Location, inc: bool) -> Location {
    if inc {
        Location::from((item.x, item.y + 1))
    } else {
        Location::from((item.x, item.y - 1))
    }
}

fn print_grid(body: &Vec<Location>) {
    println!("{:?}", PrettyTail(body));
}

fn print_tail_loc(tail_locs: &HashSet<Location>) {
    println!("== Final ==");
    println!("{:?}", PrettyBody(tail_locs));
}

fn execute_instruction_on_body(
    dir: &str,
    body: &Vec<Location>,
    tail_locs: &mut HashSet<Location>,
) -> Vec<Location> {
    let mut new_body = body.clone();

    //move head
    let head = body.first().unwrap();
    new_body[0] = move_head(dir, head);

    //check each body part
    body.iter().enumerate().for_each(|(i, _)| {
        if i < body.len() - 1 {
            let first = &new_body[i];
            let second = &body[i + 1];
            let is_tail = i == body.len() - 2;

            let tail = move_tail(dir, first, second, is_tail, tail_locs);

            if let Some(new_tail) = tail {
                new_body[i + 1] = new_tail;
            }
        }
    });
    new_body
}

fn move_head(dir: &str, head: &Location) -> Location {
    match dir {
        "R" => move_horizontal(&head, true),
        "L" => move_horizontal(&head, false),
        "U" => move_vertical(&head, true),
        "D" => move_vertical(&head, false),
        _ => panic!("no"),
    }
}

fn move_tail(
    dir: &str,
    head: &Location,
    tail: &Location,
    is_tail: bool,
    tail_locs: &mut HashSet<Location>,
) -> Option<Location> {
    let mut new_tail = None;
    let distance = distance_between_points(&head, &tail);

    if distance > 1.0 {
        // print_grid(&vec![head.clone(), tail.clone()]);
    }

    if distance >= 2.0 {
        // println!("{dir}  (before)tail: (x: {}, y: {})", tail.x, tail.y);
        let updated = match dir {
            "R" => {
                if head.y != tail.y {
                    let sub = move_vertical(&tail, head.y - tail.y > 0);
                    move_horizontal(&sub, true)
                } else {
                    move_horizontal(&tail, true)
                }
            }
            "L" => {
                if head.y != tail.y {
                    let sub = move_vertical(&tail, head.y - tail.y > 0);
                    move_horizontal(&sub, false)
                } else {
                    move_horizontal(&tail, false)
                }
            }
            "U" => {
                if head.x != tail.x {
                    let sub = move_horizontal(&tail, head.x - tail.x > 0);
                    move_vertical(&sub, true)
                } else {
                    move_vertical(&tail, true)
                }
            }
            "D" => {
                if head.x != tail.x {
                    let sub = move_horizontal(&tail, head.x - tail.x > 0);
                    move_vertical(&sub, false)
                } else {
                    move_vertical(&tail, false)
                }
            }
            _ => panic!("no"),
        };
        if is_tail {
            tail_locs.insert(updated.clone());
        }
        // println!("  tail: ({updated:?})");

        new_tail = Some(updated);
    }

    if distance > 1.0 {
        // print_grid(&vec![head.clone(), new_tail.unwrap().clone()]);
        // panic!();
    }

    new_tail
}

fn follow_instructions_and_record_unique_tail_locs(
    instructions: &Instructions,
    length: usize,
) -> usize {
    let mut body = vec![Location::new(0, 0); length];
    let mut tail_locs: HashSet<Location> = HashSet::new();
    tail_locs.insert(body.last().unwrap().clone());
    instructions.iter().for_each(|(dir, count)| {
        println!("{dir} {count}");
        for i in 0..*count {
            body = execute_instruction_on_body(dir, &body, &mut tail_locs);
            if dir == "L" && *count == 8 {
                println!("{dir} {count}/{}", i + 1);
                print_grid(&body);
            }
        }

        if dir == "L" && *count == 8 {
            panic!();
        }
        print_grid(&body);
    });
    print_tail_loc(&tail_locs);
    tail_locs.len()
}

pub fn part_1(input: &str) -> usize {
    let instructions = build_instructions_from_input_string(input);
    follow_instructions_and_record_unique_tail_locs(&instructions, 2)
}

pub fn part_2(input: &str) -> usize {
    let instructions = build_instructions_from_input_string(input);
    follow_instructions_and_record_unique_tail_locs(&instructions, 10)
}

pub fn run() {
    println!("Day 9");

    // {
    //     let input_string = include_str!("sample");
    //     println!("Part 1 - sample");
    //     let result = part_1(input_string);
    //     println!("  Result: {result}");
    //     println!("  Expected: 13\n")
    // }

    // {
    //     let input_string = include_str!("input");
    //     println!("Part 1 - input");
    //     let result = part_1(input_string);
    //     println!("  Result: {result}");
    //     println!("  Expected: 6522\n");
    // }

    // {
    //     let input_string = include_str!("sample");
    //     println!("Part 2 - sample");
    //     let result = part_2(input_string);
    //     println!("  Result: {result}");
    //     println!("  Expected: 1\n");
    // }

    {
        let input_string = include_str!("sample-1");
        println!("Part 2 - sample-1 ");
        let result = part_2(input_string);
        println!("  Result: {result}");
        println!("  Expected: 36\n");
    }
}

#[cfg(test)]
mod tests {
    use super::Location;
    use std::collections::HashSet;

    #[test]
    fn test_instruction_for_down() {
        let mut tail_locs: HashSet<Location> = HashSet::new();
        let head = Location::from((0, 0));
        let tail = Location::from((0, 0));
        match &super::execute_instruction_on_body("D", &vec![head, tail], &mut tail_locs)[..] {
            [new_head, _] => {
                assert_eq!(*new_head, Location::from((0, -1)));
            }
            _ => assert!(false, "failed"),
        };
    }

    #[test]
    fn test_instruction_for_down_with_tail() {
        let mut tail_locs: HashSet<Location> = HashSet::new();
        let head = Location::from((0, -1));
        let tail = Location::from((0, 0));
        match &super::execute_instruction_on_body("D", &vec![head, tail], &mut tail_locs)[..] {
            [new_head, new_tail] => {
                assert_eq!(*new_head, Location::from((0, -2)));
                assert_eq!(*new_tail, Location::from((0, -1)));
            }
            _ => assert!(false, "failed"),
        };
    }
    /*
    #[test]
    fn test_instruction_for_up() {
        let mut tail_locs: HashSet<Location> = HashSet::new();
        let head = Location::from((0, 0));
        let tail = Location::from((0, 0));
        let (new_head, _) =
            super::execute_instruction("U", &head, &tail, &mut tail_locs, true, true);
        assert_eq!(new_head, Location::from((0, 1)));
    }

    #[test]
    fn test_instruction_for_up_with_tail() {
        let mut tail_locs: HashSet<Location> = HashSet::new();
        let head = Location::from((0, 1));
        let tail = Location::from((0, 0));
        let (new_head, new_tail) =
            super::execute_instruction("U", &head, &tail, &mut tail_locs, true, true);
        assert_eq!(new_head, Location::from((0, 2)));
        assert_eq!(new_tail, Some(Location::from((0, 1))));
    }

    #[test]
    fn test_instruction_for_left() {
        let mut tail_locs: HashSet<Location> = HashSet::new();
        let head = Location::from((0, 0));
        let tail = Location::from((0, 0));
        let (new_head, _) =
            super::execute_instruction("L", &head, &tail, &mut tail_locs, true, true);
        assert_eq!(new_head, Location::from((-1, 0)));
    }

    #[test]
    fn test_instruction_for_left_with_tail() {
        let mut tail_locs: HashSet<Location> = HashSet::new();
        let head = Location::from((-1, 0));
        let tail = Location::from((0, 0));
        let (new_head, new_tail) =
            super::execute_instruction("L", &head, &tail, &mut tail_locs, true, true);
        assert_eq!(new_head, Location::from((-2, 0)));
        assert_eq!(new_tail, Some(Location::from((-1, 0))));
    }

    #[test]
    fn test_instruction_for_right() {
        let mut tail_locs: HashSet<Location> = HashSet::new();
        let head = Location::from((0, 0));
        let tail = Location::from((0, 0));
        let (new_head, _) =
            super::execute_instruction("R", &head, &tail, &mut tail_locs, true, true);
        assert_eq!(new_head, Location::from((1, 0)));
    }

    #[test]
    fn test_instruction_for_right_with_tail() {
        let mut tail_locs: HashSet<Location> = HashSet::new();
        let head = Location::from((1, 0));
        let tail = Location::from((0, 0));
        let (new_head, new_tail) =
            super::execute_instruction("R", &head, &tail, &mut tail_locs, true, true);
        assert_eq!(new_head, Location::from((2, 0)));
        assert_eq!(new_tail, Some(Location::from((1, 0))));
    }

    #[test]
    fn test_instruction_for_right_down() {
        let mut tail_locs: HashSet<Location> = HashSet::new();
        let head = Location::from((1, 0));
        let tail = Location::from((0, 0));
        let (new_head, _) =
            super::execute_instruction("D", &head, &tail, &mut tail_locs, true, true);
        assert_eq!(new_head, Location::from((1, -1)));
    }

    #[test]
    fn test_instruction_for_right_down_with_tail() {
        let mut tail_locs: HashSet<Location> = HashSet::new();
        let head = Location::from((1, -1));
        let tail = Location::from((0, 0));
        let (new_head, new_tail) =
            super::execute_instruction("D", &head, &tail, &mut tail_locs, true, true);
        assert_eq!(new_head, Location::from((1, -2)));
        assert_eq!(new_tail, Some(Location::from((1, -1))));
    }*/
}
