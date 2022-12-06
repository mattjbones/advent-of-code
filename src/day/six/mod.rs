use std::collections::HashSet;

pub fn run() -> Result<(), std::io::Error> {
    println!("Day 6");

    {
        println!("Part 1");
        let buffer_size = 4;
        for line in include_str!("input").lines() {
            print!("{line}");
            let mut chars = line.chars();
            let mut buff = Vec::from(chars.next_chunk::<1>().unwrap());
            let mut count = 0;
            for char in chars {
                if buff.len() < buffer_size {
                    buff.push(char);
                } else {
                    let mut set: HashSet<&char> = HashSet::new();
                    buff.iter().for_each(|c| {
                        set.insert(c);
                    });
                    if set.len() == buffer_size {
                        break;
                    }
                    buff.push(char);
                    buff.remove(0);
                }

                count += 1;
            }
            print!(", maker at {}\n", count + 1);
        }
    }

    {
        println!("Part 2");
        let buffer_size = 14;
        for line in include_str!("input").lines() {
            print!("{line}");
            let mut chars = line.chars();
            let mut buff = Vec::from(chars.next_chunk::<1>().unwrap());
            let mut count = 0;
            for char in chars {
                if buff.len() < buffer_size {
                    buff.push(char);
                } else {
                    let mut set: HashSet<&char> = HashSet::new();
                    buff.iter().for_each(|c| {
                        set.insert(c);
                    });
                    if set.len() == buffer_size {
                        break;
                    }
                    buff.push(char);
                    buff.remove(0);
                }

                count += 1;
            }
            print!(", maker at {}\n", count + 1);
        }
    }

    Ok(())
}
