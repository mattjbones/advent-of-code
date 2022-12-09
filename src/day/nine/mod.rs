use std::collections::HashSet;

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

fn distance_between_points(first: &Location, second: &Location) -> usize {
    (((first.x - second.x).pow(2) + (first.y - second.y).pow(2)) as f32).sqrt() as usize
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

fn execute_instruction(
    dir: &str,
    head: &Location,
    tail: &Location,
    tail_locs: &mut HashSet<Location>,
) -> (Location, Option<Location>) {
    let mut new_tail = None;

    let new_head = match dir {
        "R" => move_horizontal(&head, true),
        "L" => move_horizontal(&head, false),
        "U" => move_vertical(&head, true),
        "D" => move_vertical(&head, false),
        _ => panic!("no"),
    };

    println!("head: (x: {}, y: {})", new_head.x, new_head.y);
    if distance_between_points(&new_head, &tail) > 1 {
        println!("{dir}  (before)tail: (x: {}, y: {})", tail.x, tail.y);
        let updated = match dir {
            "R" => {
                if head.y != tail.y {
                    let sub = move_vertical(&tail, head.y - tail.y > 0);
                    println!("sub {sub:?}");
                    move_horizontal(&sub, true)
                } else {
                    move_horizontal(&tail, true)
                }
            }

            "L" => {
                if head.y != tail.y {
                    let sub = move_vertical(&tail, head.y - tail.y > 0);
                    println!("sub {sub:?}");
                    move_horizontal(&sub, false)
                } else {
                    move_horizontal(&tail, false)
                }
            }
            "U" => {
                if head.x != tail.x {
                    let sub = move_horizontal(&tail, head.x - tail.x > 0);
                    println!("sub {sub:?}");
                    move_vertical(&sub, true)
                } else {
                    move_vertical(&tail, true)
                }
            }
            "D" => {
                if head.x != tail.x {
                    let sub = move_horizontal(&tail, head.x - tail.x > 0);
                    println!("sub {sub:?}");
                    move_vertical(&sub, false)
                } else {
                    move_vertical(&tail, false)
                }
            }
            _ => panic!("no"),
        };
        tail_locs.insert(updated.clone());
        // println!("{:#?}", tail_locs);
        println!("  tail: ({updated:?})");

        // if tail_locs.len() == 3 {
        //     println!("");
        //     panic!();
        // }

        new_tail = Some(updated);
    }

    (new_head, new_tail)
}

//head = x,y
fn follow_instructions_and_record_unique_tail_locs(instructions: &Instructions) -> usize {
    let mut head: Location = Location::new(0, 0);
    let mut tail: Location = head.clone();
    let mut tail_locs: HashSet<Location> = HashSet::new();
    tail_locs.insert(tail.clone());
    instructions.iter().for_each(|(dir, count)| {
        println!("{dir} {count}");
        for _ in 0..*count {
            let updated = execute_instruction(dir, &head, &tail, &mut tail_locs);
            head = updated.0;
            if let Some(new_tail) = updated.1 {
                tail = new_tail;
            }
            println!("");
        }
    });
    tail_locs.len()
}

pub fn part_1(input: &str) -> usize {
    let instructions = build_instructions_from_input_string(input);
    follow_instructions_and_record_unique_tail_locs(&instructions)
}

pub fn run() {
    println!("Day 9");

    {
        let input_string = include_str!("sample");
        println!("Part 1 - sample");
        let result = part_1(input_string);
        println!("  Result: {result}");
        println!("  Expected: 13\n")
    }

    {
        let input_string = include_str!("input");
        println!("Part 1 - input");
        let result = part_1(input_string);
        println!("  Result: {result} (prev: 6690)");
        println!("  Expected: ....\n");
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
        let (new_head, _) = super::execute_instruction("D", &head, &tail, &mut tail_locs);
        assert_eq!(new_head, Location::from((0, -1)));
    }

    #[test]
    fn test_instruction_for_down_with_tail() {
        let mut tail_locs: HashSet<Location> = HashSet::new();
        let head = Location::from((0, -1));
        let tail = Location::from((0, 0));
        let (new_head, new_tail) = super::execute_instruction("D", &head, &tail, &mut tail_locs);
        assert_eq!(new_head, Location::from((0, -2)));
        assert_eq!(new_tail, Some(Location::from((0, -1))));
    }

    #[test]
    fn test_instruction_for_up() {
        let mut tail_locs: HashSet<Location> = HashSet::new();
        let head = Location::from((0, 0));
        let tail = Location::from((0, 0));
        let (new_head, _) = super::execute_instruction("U", &head, &tail, &mut tail_locs);
        assert_eq!(new_head, Location::from((0, 1)));
    }

    #[test]
    fn test_instruction_for_up_with_tail() {
        let mut tail_locs: HashSet<Location> = HashSet::new();
        let head = Location::from((0, 1));
        let tail = Location::from((0, 0));
        let (new_head, new_tail) = super::execute_instruction("U", &head, &tail, &mut tail_locs);
        assert_eq!(new_head, Location::from((0, 2)));
        assert_eq!(new_tail, Some(Location::from((0, 1))));
    }

    #[test]
    fn test_instruction_for_left() {
        let mut tail_locs: HashSet<Location> = HashSet::new();
        let head = Location::from((0, 0));
        let tail = Location::from((0, 0));
        let (new_head, _) = super::execute_instruction("L", &head, &tail, &mut tail_locs);
        assert_eq!(new_head, Location::from((-1, 0)));
    }

    #[test]
    fn test_instruction_for_left_with_tail() {
        let mut tail_locs: HashSet<Location> = HashSet::new();
        let head = Location::from((-1, 0));
        let tail = Location::from((0, 0));
        let (new_head, new_tail) = super::execute_instruction("L", &head, &tail, &mut tail_locs);
        assert_eq!(new_head, Location::from((-2, 0)));
        assert_eq!(new_tail, Some(Location::from((-1, 0))));
    }

    #[test]
    fn test_instruction_for_right() {
        let mut tail_locs: HashSet<Location> = HashSet::new();
        let head = Location::from((0, 0));
        let tail = Location::from((0, 0));
        let (new_head, _) = super::execute_instruction("R", &head, &tail, &mut tail_locs);
        assert_eq!(new_head, Location::from((1, 0)));
    }

    #[test]
    fn test_instruction_for_right_with_tail() {
        let mut tail_locs: HashSet<Location> = HashSet::new();
        let head = Location::from((1, 0));
        let tail = Location::from((0, 0));
        let (new_head, new_tail) = super::execute_instruction("R", &head, &tail, &mut tail_locs);
        assert_eq!(new_head, Location::from((2, 0)));
        assert_eq!(new_tail, Some(Location::from((1, 0))));
    }

    #[test]
    fn test_instruction_for_right_down() {
        let mut tail_locs: HashSet<Location> = HashSet::new();
        let head = Location::from((1, 0));
        let tail = Location::from((0, 0));
        let (new_head, _) = super::execute_instruction("D", &head, &tail, &mut tail_locs);
        assert_eq!(new_head, Location::from((1, -1)));
    }

    #[test]
    fn test_instruction_for_right_down_with_tail() {
        let mut tail_locs: HashSet<Location> = HashSet::new();
        let head = Location::from((1, -1));
        let tail = Location::from((0, 0));
        let (new_head, new_tail) = super::execute_instruction("D", &head, &tail, &mut tail_locs);
        assert_eq!(new_head, Location::from((1, -2)));
        assert_eq!(new_tail, Some(Location::from((1, -1))));
    }

    // #[test]
    // fn test_instruction_for_right_up() {
    //     let mut tail_locs: HashSet<Location> = HashSet::new();
    //     let head = Location::from((1, 0));
    //     let tail = Location::from((0, 0));
    //     let (new_head, _) = super::execute_instruction("D", &head, &tail, &mut tail_locs);
    //     assert_eq!(new_head, Location::from((1, -1)));
    // }

    // #[test]
    // fn test_instruction_for_right_up_with_tail() {
    //     let mut tail_locs: HashSet<Location> = HashSet::new();
    //     let head = Location::from((1, -1));
    //     let tail = Location::from((0, 0));
    //     let (new_head, new_tail) = super::execute_instruction("R", &head, &tail, &mut tail_locs);
    //     assert_eq!(new_head, Location::from((2, -1)));
    //     assert_eq!(new_tail, Some(Location::from((1, -1))));
    // }
}
