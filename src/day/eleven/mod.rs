mod monkey;

use monkey::Monkey;

fn parse_input_string(input: &str) -> Vec<Monkey> {
    let monkey_lines: Vec<&str> = input.split("\n\n").collect();
    monkey_lines
        .iter()
        .map(|monkey_line| Monkey::new(&monkey_line))
        .collect::<Vec<Monkey>>()
}

fn print_items(monkeys: &Vec<Monkey>) {
    for (i, monkey) in monkeys.iter().enumerate() {
        println!("Monkey {i}: {:?}", monkey.items);
    }
}

fn play_round(monkeys: &mut Vec<Monkey>, inspects: &mut Vec<i32>) {
    for i in 0..monkeys.len() {
        let monkey = &mut monkeys[i].clone();
        if monkey.items.len() == 0 {
            continue;
        }

        while let Some(item) = monkey.take_first() {
            let new_item = (monkey.operation)(&item) / 3;
            let test_result = (monkey.test)(&new_item);
            let target_monkey = (monkey.test_action)(test_result) as usize;
            monkeys[target_monkey].items.push_back(new_item);
            monkeys[i] = monkey.to_owned();
            inspects[i] += 1;
        }
    }
    print_items(&monkeys);
}

fn part_1(input: &str) -> i32 {
    let monkeys = &mut parse_input_string(input);
    let mut inspects = vec![0; monkeys.len()];
    for i in 0..20 {
        println!("== Round {i} ==");
        play_round(monkeys, &mut inspects);
        println!("");
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
        println!("  Expected 10605");
    }

    {
        println!("Part 1 - input");
        let monkey_business_total = part_1(include_str!("input"));
        println!("  Result {}", monkey_business_total);
        println!("  Expected 54036");
    }
}
