use types::*;

pub mod types;

pub fn run() {
    println!("Day 5");

    {
        println!("Part 1");
        let mut lines = include_str!("input-1").lines();
        let mut line = lines.next().unwrap();
        let dock = &mut Dock::new(line.len() / 4 + 1);
        while line.len() > 0 {
            let line_chars = line.chars();
            let mut buff: Vec<char> = Vec::new();
            for char in line_chars {
                if buff.len() < 5 {
                    buff.push(char);
                    if buff.len() == 4 {
                        let container = Container::try_from(buff.as_slice()).ok();
                        dock.push_next(container);
                        buff.clear();
                    }
                }
            }
            dock.push_next(Container::try_from(buff.as_slice()).ok());
            line = lines.next().unwrap();
        }

        // removing first line
        dock.prune();

        // mentally I want to pop from the top of the list
        // if we don't flip the stacks are the wrong way up
        dock.flip();

        while let Some(line) = lines.next() {
            let instructions: Vec<&str> = line.split(" ").collect();
            let action = instructions[0];
            if action != "move" {
                panic!("unknown instruction");
            }
            match instructions[..] {
                [_, count, _, from, _, to] => {
                    dock.move_containers_9000(
                        from.parse::<usize>().unwrap(),
                        to.parse::<usize>().unwrap(),
                        count.parse::<usize>().unwrap(),
                    );
                }
                _ => panic!("unknown instruction format"),
            }
        }

        println!("Tops: {}", dock.tops());
    }

    {
        println!("\nPart 2");
        let mut lines = include_str!("input-1").lines();
        let mut line = lines.next().unwrap();
        let dock = &mut Dock::new(line.len() / 4 + 1);
        while line.len() > 0 {
            let line_chars = line.chars();
            let mut buff: Vec<char> = Vec::new();
            for char in line_chars {
                if buff.len() < 5 {
                    buff.push(char);
                    if buff.len() == 4 {
                        let container = Container::try_from(buff.as_slice()).ok();
                        dock.push_next(container);
                        buff.clear();
                    }
                }
            }
            dock.push_next(Container::try_from(buff.as_slice()).ok());
            line = lines.next().unwrap();
        }

        // removing first line
        dock.prune();

        // mentally I want to pop from the top of the list
        // if we don't flip the stacks are the wrong way up
        dock.flip();

        // println!("{:#?}", dock);

        while let Some(line) = lines.next() {
            let instructions: Vec<&str> = line.split(" ").collect();
            let action = instructions[0];
            if action != "move" {
                panic!("unknown instruction");
            }
            match instructions[..] {
                [_, count, _, from, _, to] => {
                    dock.move_containers_9001(
                        from.parse::<usize>().unwrap(),
                        to.parse::<usize>().unwrap(),
                        count.parse::<usize>().unwrap(),
                    );
                }
                _ => panic!("unknown instruction format"),
            }
        }

        println!("Tops: {}", dock.tops());
    }
}
