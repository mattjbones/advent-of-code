use serde_json::Value;

pub fn run() {
    println!("Day 13");
    // {
    //     println!("Part 1 - Sample");
    //     let result = part_1(include_str!("sample"));
    //     println!(" Result {result}");
    //     println!(" Expected 13");
    // }

    {
        println!("Part 1 - Input");
        let result = part_1(include_str!("input"));
        println!(" Result {result} (prev: 2548)");
        println!(" Expected ...");
    }
}

fn part_1(input: &str) -> usize {
    let pair_strings = input.split("\n\n").collect::<Vec<&str>>();
    let pairs = pair_strings
        .iter()
        .enumerate()
        .map(|(no, pairs)| {
            let pair = pairs.split("\n").collect::<Vec<&str>>();
            println!("== Pair {} ==", no + 1);

            let left = pair[0];
            let right = pair[1];

            let left_json: Value = serde_json::from_str(left).unwrap();
            let right_json: Value = serde_json::from_str(right).unwrap();

            println!("{}", left_json);
            println!("{}", right_json);

            let left_arr = left_json.as_array().unwrap().clone();
            let right_arr = right_json.as_array().unwrap().clone();

            println!("- Compare {} vs {}", left_json, right_json);
            let mut level = 1;
            let valid = compare_arrays(&left_arr, &right_arr, false, &mut level);
            level += 1;
            if valid {
                print!("{}", String::from_iter(vec![" "; level]));
                print!("- Left side is smaller RIGHT");
                println!()
            } else {
                print!("{}", String::from_iter(vec![" "; level]));
                println!("- Right side is smaller, WRONG");
                println!()
            }

            if no == 10 {
                // panic!();
            }
            println!();
            valid
        })
        .collect::<Vec<bool>>();

    // println!("pairs: {:?}", pairs);
    pairs.iter().enumerate().fold(0, |acc, (i, valid)| {
        if *valid {
            // println!("indicie {}", i + 1);
            acc + i + 1
        } else {
            acc
        }
    })
}

fn compare_arrays(
    left_input: &Vec<Value>,
    right_input: &Vec<Value>,
    converted: bool,
    level: &mut usize,
) -> bool {
    let mut valid = true;
    if left_input.len() == 0 && right_input.len() > 0 {
        return valid;
    }

    let mut left = left_input.clone();
    left.reverse();

    let mut right = right_input.clone();
    right.reverse();
    while let Some(item) = left.pop() {
        if let Some(right_item) = right.pop() {
            print!("{}", String::from_iter(vec![" "; *level]));
            print!("- Compare {} vs {}", item, right_item);
            println!();
            valid = comparator(&item, &right_item, level);
            if !valid {
                break;
            }
        } else {
            *level += 1;
            // println!("No more right {} left ({:?})", right.len(), item);
            if !converted {
                print!("{}", String::from_iter(vec![" "; *level]));
                print!("- Right side ran out of items");
                println!();
                valid = false;
            }
            break;
        }
        *level -= 1;
        // println!("({}) next (item: {}, right: {})", level, item, right.len());
    }
    valid
}

fn comparator(left: &Value, right: &Value, level: &mut usize) -> bool {
    let mut is_valid = true;
    if left.is_number() && right.is_number() {
        // println!("comparing left {:?} and right {:?}", left, right);
        *level += 1;
        if left.as_u64().unwrap() > right.as_u64().unwrap() {
            is_valid = false;
        }
    } else if left.is_array() && right.is_array() {
        *level += 1;

        is_valid = compare_arrays(
            left.as_array().unwrap(),
            right.as_array().unwrap(),
            false,
            level,
        );
    } else if left.is_array() && right.is_number() {
        print!("{}", String::from_iter(vec![" "; *level]));
        print!(
            "- Mixed types; convert right to [{}] and retry comparison",
            right.as_u64().unwrap()
        );
        println!();
        *level += 1;
        let left_array = left.as_array().unwrap();
        is_valid = compare_arrays(left_array, &Vec::from([right.clone()]), true, level);
    } else if right.is_array() && left.is_number() {
        print!("{}", String::from_iter(vec![" "; *level]));
        print!(
            "- Mixed types; convert left to [{}] and retry comparison",
            left.as_u64().unwrap()
        );
        println!();
        *level += 1;
        let right_array = right.as_array().unwrap();
        is_valid = compare_arrays(&Vec::from([left.clone()]), right_array, true, level);
    } else {
        panic!("wot?");
    }

    is_valid
}

#[cfg(test)]
mod tests {

    use std::str::FromStr;

    use super::*;

    #[test]
    fn test_comparator_for_number_array() {
        // 4 vs [] -> [4] vs [] -> (mixed types) valid
        let number = Value::from(4);
        let array = Value::from_str("[]").unwrap();
        let is_valid = comparator(&number, &array, &mut 0);
        assert!(is_valid);
    }

    #[test]
    fn test_comparator_for_array_number() {
        // [] vs 4 -> [] vs [4] -> left runs out (valid)
        let number = Value::from(4);
        let array = Value::from_str("[]").unwrap();
        let is_valid = comparator(&array, &number, &mut 0);
        assert!(is_valid);
    }

    #[test]
    fn test_comparator_for_array_array_number() {
        // [[]] vs [4] -> [] vs 4 -> [] vs [4] -> left runs out (valid)
        let number = Value::from(4);
        let value_array = Value::from_str("[[]]").unwrap();
        let array = value_array.as_array().unwrap();
        let is_valid = compare_arrays(&array, &Vec::from([number]), true, &mut 0);
        assert!(is_valid);
    }

    #[test]
    fn test_comparator_for_number_array_array() {
        //[4] vs [[]] -> 4 vs [] -> [4] vs [] ->  mixed types
        let number = Value::from(4);
        let value_array = Value::from_str("[[]]").unwrap();
        let array = value_array.as_array().unwrap();
        let is_valid = compare_arrays(&Vec::from([number]), &array, true, &mut 0);
        assert!(is_valid);
    }

    #[test]
    fn test_left_array_empty_right_empty() {
        // [] vs [[]] -> left runs out (valid)

        let empty = Value::from_str("[]").unwrap();
        let array = Value::from_str("[[]]").unwrap();

        let is_valid = comparator(&empty, &array, &mut 0);
        assert!(is_valid);
    }

    #[test]
    fn test_left_empty_right_empty_array() {
        // [[]] vs [] -> right runs out (invalid)
        let empty = Value::from_str("[]").unwrap();
        let array = Value::from_str("[[]]").unwrap();

        let is_valid = comparator(&array, &empty, &mut 0);
        assert!(!is_valid);
    }

    #[test]
    fn test_pair_seven() {
        let left_string = "[
            [
                [],2
            ]
        ]";

        let right_string = "[
            [
                [
                   [3,4,1,3],6,7,[7,0,7,7,5,5]
                ]
            ]
        ]";

        let left_json: Value = serde_json::from_str(left_string).unwrap();
        let right_json: Value = serde_json::from_str(right_string).unwrap();

        let is_valid = comparator(&left_json, &right_json, &mut 0);
        assert!(!is_valid);
    }
}
