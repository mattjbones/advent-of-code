use std::cell::RefCell;

#[derive(Debug, PartialEq, Clone)]
enum Item {
    Dir(Box<Directory>),
    File((String, String)),
}

#[derive(Debug, PartialEq, Clone)]
struct Directory {
    path: String,
    contents: Vec<Item>,
    parent: Option<Box<Directory>>,
}

impl std::fmt::Display for Directory {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Directory")
            .field("path", &self.path)
            .field("contents", &self.contents)
            .finish()
    }
}

impl Directory {
    fn new(path: &str, parent: Option<Box<Directory>>) -> Self {
        Self {
            path: path.to_string(),
            contents: Vec::new(),
            parent,
        }
    }

    fn find_dir(&mut self, target_path: &str) -> Option<&mut Box<Directory>> {
        let mut found: Option<&mut Box<Directory>> = None;
        let contents = &mut self.contents;
        for item in contents {
            match item {
                Item::Dir(dir) => {
                    if dir.path.eq(target_path) {
                        found = Some(dir)
                    } else {
                        found = dir.find_dir(target_path)
                    }
                }
                _ => (),
            }
        }
        found
    }

    fn add_item(&mut self, item: Item) {
        self.contents.push(item);
    }
}

pub fn run() {
    println!("Day 7");

    {
        println!("Part 1");
        let mut current_dir: &str = "/";
        let outer_root = &mut Box::new(Directory::new(current_dir, None));
        for line in include_str!("sample").lines() {
            let mut root = Box::clone(&outer_root);
            // println!("{line}");
            let parts: Vec<&str> = line.split(" ").collect();
            if line.starts_with("$") {
                //instruction
                let instruction = match parts[..] {
                    [_, instruction] => [Some(instruction), None],
                    [_, instruction, argument] => [Some(instruction), Some(argument)],
                    _ => {
                        panic!("unknown instruction");
                    }
                };

                match instruction[..] {
                    [Some("cd"), Some(path)] => {
                        println!("cd: {path}");
                        current_dir = match path {
                            "/" => "/",
                            ".." => {
                                let mut new_parent = "";
                                if let Some(dir) = root.find_dir(&current_dir) {
                                    if let Some(parent) = &dir.parent {
                                        println!("current: {current_dir}");
                                        println!("cd .. ");

                                        //THIS LINE FAILS... why tho?
                                        //I want to assign a copy of the string (so I can find the dir again later)
                                        //I should just copy a reference to the current node.

                                        new_parent = &parent.path.clone()

                                        // println!("parent: {}", parent.path);

                                        // pr
                                    } else {
                                        panic!("cannot find match");
                                    }
                                } else {
                                    panic!("cannot find match");
                                }
                                new_parent
                                // panic!("up! \n{:#?}", root);
                            }
                            _ => {
                                if current_dir == "/" {
                                    root.add_item(Item::Dir(Box::new(Directory::new(
                                        &path.to_string(),
                                        Some(Box::clone(&root)),
                                    ))));
                                } else {
                                    println!("child dir: {}", current_dir);
                                    if let Some(dir) = &mut root.find_dir(&current_dir) {
                                        dir.contents.push(Item::Dir(Box::new(Directory::new(
                                            &path.to_string(),
                                            Some(Box::clone(&dir)),
                                        ))));
                                    }
                                }
                                path
                            }
                        };
                    }
                    [Some("ls"), _] => {
                        println!("ls");
                    }
                    _ => panic!("unknown instruction"),
                }
            } else {
                // result
                match parts[..] {
                    ["dir", dir_name] => {
                        println!("directory: {dir_name}")
                    }
                    [filesize, filename] => {
                        println!("filesize: {filesize}, filename: {filename}")
                    }
                    _ => panic!("unknown result"),
                }
            }
            *outer_root = root;
        }
        println!("{:#?}", outer_root);
    }
}
