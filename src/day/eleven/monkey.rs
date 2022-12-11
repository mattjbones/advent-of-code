use std::{collections::VecDeque, fmt::Debug, rc::Rc};

type Operation = Rc<dyn Fn(&i64) -> i64>;
type TestFn = Rc<dyn Fn(&i64) -> bool>;
type TestAction = Rc<dyn Fn(bool) -> u32>;

#[derive(Clone)]
pub struct Monkey {
    pub items: VecDeque<i64>,
    pub operation: Operation,
    pub test: TestFn,
    pub test_action: TestAction,
    pub worry_level_factor: i64,
    pub test_val: i64,
}

impl Debug for Monkey {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Monkey")
            .field("items", &self.items)
            // .field("operation", &self.operation)
            // .field("test", &self.test)
            // .field("test_action", &self.test_action)
            .finish()
    }
}

impl Monkey {
    pub fn take_first(&mut self) -> Option<i64> {
        VecDeque::pop_front(&mut self.items)
    }

    pub fn new(input: &str, worry_level_factor: i64) -> Self {
        let lines: Vec<&str> = input.lines().into_iter().collect();
        let (first, rest) = lines.split_first().unwrap();
        if !first.contains("Monkey") {
            panic!("Unknown input");
        }

        let mut empty = Self {
            items: VecDeque::new(),
            test: Rc::new(|_| false),
            operation: Rc::new(|_| 0),
            test_action: Rc::new(|_| 0),
            worry_level_factor,
            test_val: 0,
        };

        for line in rest {
            if let Some(items) = Self::parse_line_for_items(line) {
                empty.items = VecDeque::from(items);
            }

            if let Some(op) = Self::parse_line_for_operation_fn(line) {
                empty.operation = op;
            }

            if let Some(test_fn) = Self::parse_line_for_test_fn(line) {
                empty.test = test_fn.0;
                empty.test_val = test_fn.1;
            }
        }
        if let Some(test_action) = Self::parse_lines_for_test_actions(rest) {
            empty.test_action = test_action;
        }

        empty
    }

    fn extract_value_from_string(line: &str, test: &str) -> Option<String> {
        if line.contains(test) {
            let indices: Vec<_> = line.match_indices(test).collect();
            Some(
                line.split_at(indices.first().unwrap().0 + test.len())
                    .1
                    .to_string(),
            )
        } else {
            None
        }
    }

    fn parse_lines_for_test_actions(rest: &[&str]) -> Option<TestAction> {
        let true_false_actions: Vec<u32> = rest
            .into_iter()
            .filter(|line| line.starts_with("    "))
            .map(|action| action.chars().last().unwrap().to_digit(10).unwrap())
            .collect();

        Some(Rc::new(move |test: bool| {
            if test {
                true_false_actions[0]
            } else {
                true_false_actions[1]
            }
        }))
    }

    fn parse_line_for_test_fn(line: &str) -> Option<(TestFn, i64)> {
        if let Some(test_string) = Self::extract_value_from_string(line, "Test:") {
            match test_string.trim().split(" ").collect::<Vec<&str>>()[..] {
                ["divisible", _, val] => {
                    let val = val.parse::<i64>();
                    let return_val = val.clone();
                    Some((
                        Rc::new(move |item: &i64| item % val.clone().unwrap() == 0),
                        return_val.unwrap(),
                    ))
                }
                _ => panic!("ded"),
            }
        } else {
            None
        }
    }

    fn parse_line_for_operation_fn(line: &str) -> Option<Operation> {
        if let Some(op_string) = Self::extract_value_from_string(line, "Operation:") {
            // println!("{op_string}");
            let operation = op_string
                .trim()
                .split("=")
                .collect::<Vec<&str>>()
                .last()
                .unwrap()
                .trim()
                .split(" ")
                .collect::<Vec<&str>>();

            match operation[..] {
                [_, "*", val] => {
                    let number_val = val.parse::<i64>();
                    Some(Rc::new(move |item: &i64| {
                        // println!("item {item} * {}", number_val.clone().unwrap_or(*item));
                        item * number_val.clone().unwrap_or(*item)
                    }))
                }
                [_, "/", val] => {
                    let number_val = val.parse::<i64>();
                    Some(Rc::new(move |item: &i64| {
                        // println!("item {item} / {}", number_val.clone().unwrap_or(*item));
                        item / number_val.clone().unwrap_or(*item)
                    }))
                }
                [_, "+", val] => {
                    let number_val = val.parse::<i64>();
                    Some(Rc::new(move |item: &i64| {
                        // println!("item {item} + {}", number_val.clone().unwrap_or(*item));
                        item + number_val.clone().unwrap_or(*item)
                    }))
                }
                [_, "-", val] => {
                    let number_val = val.parse::<i64>();
                    Some(Rc::new(move |item: &i64| {
                        // println!("item {item} - {}", number_val.clone().unwrap_or(*item));
                        item - number_val.clone().unwrap_or(*item)
                    }))
                }
                _ => panic!("no idea"),
            }
        } else {
            None
        }
    }

    fn parse_line_for_items(line: &str) -> Option<Vec<i64>> {
        if let Some(items_string) = Self::extract_value_from_string(line, "Starting items:") {
            Some(
                items_string
                    .split(",")
                    .map(|item| item.trim().parse::<i64>().unwrap())
                    .collect(),
            )
        } else {
            None
        }
    }
}
