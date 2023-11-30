mod monkey;

use monkey::Monkey;

fn parse_input_string(input: &str, worry_level_factor: i64) -> Vec<Monkey> {
    let monkey_lines: Vec<&str> = input.split("\n\n").collect();
    monkey_lines
        .iter()
        .map(|monkey_line| Monkey::new(&monkey_line, worry_level_factor))
        .collect::<Vec<Monkey>>()
}

// fn print_items(monkeys: &Vec<Monkey>) {
//     for (i, monkey) in monkeys.iter().enumerate() {
//         println!("Monkey {i}: {:?}", monkey.items);
//     }
// }

// fn print_inspects(inspects: &Vec<i64>) {
//     for (i, inspect) in inspects.iter().enumerate() {
//         println!("Monkey {i} inspected items {:?} times.", inspect);
//     }
// }

fn play_round(monkeys: &mut Vec<Monkey>, inspects: &mut Vec<i64>, test_total: i64) {
    for i in 0..monkeys.len() {
        let monkey = &mut monkeys[i].clone();
        if monkey.items.len() == 0 {
            continue;
        }

        while let Some(item) = monkey.take_first() {
            let new_item = (monkey.operation)(&(item)) / monkey.worry_level_factor;
            let test_result = (monkey.test)(&new_item);
            let target_monkey = (monkey.test_action)(test_result) as usize;
            monkeys[target_monkey]
                .items
                .push_back(new_item % test_total);
            monkeys[i] = monkey.to_owned();
            inspects[i] += 1;
        }
    }
    // print_items(&monkeys);
    // print_inspects(&inspects);
}

fn part_1(input: &str) -> i64 {
    let monkeys = &mut parse_input_string(input, 3);
    let test_total = monkeys
        .iter()
        .map(|monk| monk.test_val)
        .reduce(|acc, test_val| {
            let new = acc * test_val;
            new
        })
        .unwrap();

    let mut inspects: Vec<i64> = vec![0; monkeys.len()];
    for _ in 0..20 {
        // println!("== round {i} ==");
        play_round(monkeys, &mut inspects, test_total);
        // println!("");
    }

    inspects.sort_by(|a, b| b.cmp(a));
    inspects[0..2]
        .iter()
        .copied()
        .reduce(|acc, item| acc * item)
        .unwrap()
        .to_owned()
}

fn part_2(input: &str) -> i64 {
    let monkeys = &mut parse_input_string(input, 1);
    let test_total = monkeys
        .iter()
        .map(|monk| monk.test_val)
        .reduce(|acc, test_val| {
            let new = acc * test_val;
            new
        })
        .unwrap();

    let mut inspects: Vec<i64> = vec![0; monkeys.len()];
    for _ in 0..10000 {
        // println!("\n== round {i} ==");
        play_round(monkeys, &mut inspects, test_total);
        // println!("");
    }

    inspects.sort_by(|a, b| b.cmp(a));
    inspects[0..2]
        .iter()
        .copied()
        .reduce(|acc, item| acc * item)
        .unwrap()
        .to_owned()
}

pub fn run() {
    println!("Day 11");

    {
        println!("Part 1 - sample");
        let monkey_business_total = part_1(include_str!("sample"));
        println!("  Result {}", monkey_business_total);
        println!("  Expected 10605\n");
    }

    {
        println!("Part 1 - input");
        let monkey_business_total = part_1(include_str!("input"));
        println!("  Result {}", monkey_business_total);
        println!("  Expected 54036\n");
    }

    {
        println!("Part 2 - sample");
        let monkey_business_total = part_2(include_str!("sample"));
        println!("  Result {}", monkey_business_total);
        println!("  Expected 2713310158\n");
    }

    {
        println!("Part 2 - input");
        let monkey_business_total = part_2(include_str!("input"));
        println!("  Result {}", monkey_business_total);
        println!("  Expected 13237873355");
    }
}
