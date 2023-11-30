use core::panic;

fn check_cycle(cycle: i32, length: i32) -> bool {
    let cycle = cycle - length;
    if cycle == 0 || cycle % 40 == 0 {
        true
    } else {
        false
    }
}

fn generate_cycles(instructions: &str) -> Vec<i32> {
    let mut cycles = Vec::new();
    let mut cycle = 0;
    let mut x = 1;
    instructions.lines().into_iter().for_each(|line| {
        match line.split(" ").collect::<Vec<&str>>()[..] {
            [op, val] => {
                if op == "addx" {
                    let mut count = 0;
                    while count < 2 {
                        cycle += 1;
                        if check_cycle(cycle, 20) {
                            cycles.push(x * cycle);
                        }
                        count += 1;
                    }
                    let val = val.parse::<i32>().unwrap();
                    x += val;
                }
            }
            [_] => {
                cycle += 1;
                if check_cycle(cycle, 20) {
                    cycles.push(x * cycle);
                }
            }
            _ => panic!("ded"),
        }
    });
    cycles
}

fn part_1(instructions: &str) -> i32 {
    let cycles = generate_cycles(instructions);
    cycles.iter().fold(0, |acc, x| acc + x)
}

fn update_line(cycle_line: &mut String, cycles: &mut Vec<String>, cycle: i32, x: i32) {
    if check_cycle(cycle, 40) {
        cycles.push(cycle_line.clone());
        cycle_line.clear();
    } else {
        let pos = cycle_line.len() as i32;
        let is_lit = x == pos || x == pos - 1 || x == pos + 1;
        if is_lit {
            cycle_line.push_str("#");
        } else {
            cycle_line.push_str(".");
        }
    }
}

fn part_2(instructions: &str) -> Vec<String> {
    let mut cycles: Vec<String> = Vec::new();
    let mut cycle_line = String::new();
    let mut cycle = 0;
    let mut x = 1;
    instructions.lines().into_iter().for_each(|line| {
        match line.split(" ").collect::<Vec<&str>>()[..] {
            [op, val] => {
                if op == "addx" {
                    let mut count = 0;
                    while count < 2 {
                        cycle += 1;
                        update_line(&mut cycle_line, &mut cycles, cycle, x);
                        count += 1;
                    }
                    let val = val.parse::<i32>().unwrap();
                    x += val;
                }
            }
            [_] => {
                cycle += 1;
                update_line(&mut cycle_line, &mut cycles, cycle, x);
            }
            _ => panic!("ded"),
        }
    });
    cycles
}

pub fn run() {
    println!("Day 10");

    {
        println!("Part 1 - sample");
        let result = part_1(&include_str!("sample").to_string());
        println!("  Result: {result}");
        println!("  Expected: 13140");
    }

    {
        println!("\nPart 1 - sample");
        let result = part_1(&include_str!("input").to_string());
        println!("  Result: {result}");
        println!("  Expected: 12560");
    }

    {
        println!("\nPart 2 - sample");
        part_2(&include_str!("sample").to_string());
        let result = part_2(&include_str!("input").to_string());
        println!("  {result:#?}");
    }

    {
        println!("\nPart 2 - input");
        let result = part_2(&include_str!("input").to_string());
        println!("  Result: PLPAFBCL");
        println!("  {result:#?}");
        println!("  Expected: PLPAFBCL");
    }
}
