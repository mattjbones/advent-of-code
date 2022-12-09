use std::fmt;

#[derive(PartialEq, Debug, Clone)]
enum Item {
    Dir(Box<Directory>),
    File((String, i32)),
}

#[derive(PartialEq, Debug, Clone)]
struct Directory {
    path: String,
    contents: Vec<Item>,
    parent: Option<String>,
    file_size: i32,
    total_size: i32,
}

struct PrettyNode(Item);

impl fmt::Debug for PrettyNode {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let this = &self.0;
        match this {
            Item::Dir(dir) => {
                writeln!(f, "(dir)")?;
                for item in &dir.contents {
                    for (index, line) in format!("{:?}", PrettyNode(item.clone()))
                        .lines()
                        .enumerate()
                    {
                        if index == 0 {
                            let name = match item {
                                Item::Dir(dir) => dir.path.clone(),
                                Item::File((name, _)) => name.to_string(),
                            };
                            writeln!(f, "{name} {line}")?;
                        } else {
                            writeln!(f, "  {line}")?;
                        }
                    }
                }
            }
            Item::File(file) => writeln!(f, "(file, size={})", file.1)?,
        }

        Ok(())
    }
}

const TARGET_SIZE: i32 = 100_000;
const TOTAL_SPACE: i32 = 70_000_000;
const MIN_UNUSED_SPACE: i32 = 30_000_000;

impl Directory {
    fn new(path: &str, parent: Option<String>) -> Self {
        Self {
            path: path.to_string(),
            contents: Vec::new(),
            parent,
            file_size: 0,
            total_size: 0,
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

    fn add_item(&mut self, item: Item, file_size: i32) {
        self.contents.push(item);
        self.file_size += file_size;
    }
}

pub fn run() {
    println!("Day 7");

    {
        println!("Part 1 - sample ");
        let root = &mut build_directory_tree_string(include_str!("sample"));

        let totals = walk_for_dir_totals(root);
        let total = totals
            .into_iter()
            .filter(|total| *total < TARGET_SIZE)
            .reduce(|acc, curr| acc + curr)
            .unwrap();

        println!("total {:?}", total);
        println!("expected {}", 95437);
    }

    {
        println!("\nPart 1 - input");
        let root = &mut build_directory_tree_string(include_str!("input"));

        let totals = walk_for_dir_totals(root);
        let total = totals
            .into_iter()
            .filter(|total| *total < TARGET_SIZE)
            .reduce(|acc, curr| acc + curr)
            .unwrap();

        println!("total {:?}", total);
        println!("expected {}", 1490523);

        let item = Item::Dir(root.to_owned());
        println!("{:?}", PrettyNode(item));
        // panic!("test");
    }

    {
        println!("\nPart 2 - sample");
        let root = &mut build_directory_tree_string(include_str!("sample"));
        let total = find_dir_to_delete(root);
        println!("total {:?}", total);
        println!("expected {}", 24933642);
    }

    {
        println!("\nPart 2 - input");
        let root = &mut build_directory_tree_string(include_str!("input"));
        let total = find_dir_to_delete(root);
        println!("total {:?}", total);
        println!("expected {}", 12390492);
    }
}

fn find_dir_to_delete(root: &mut Box<Directory>) -> i32 {
    let mut totals = walk_for_dir_totals(root);
    totals.sort();
    let dir_totals = totals.split_at(totals.len() - 1).0;
    println!("totals: {totals:?}");
    let used_space = root.total_size;
    let free_space = dbg!(TOTAL_SPACE - used_space);
    let target_dirs: Vec<i32> = dir_totals
        .into_iter()
        .rev()
        .copied()
        .filter(|total| *total + free_space > MIN_UNUSED_SPACE)
        .collect();

    println!("{target_dirs:?}");
    target_dirs.first().unwrap().to_owned()
}

fn build_directory_tree_string(tree: &str) -> Box<Directory> {
    let mut current_dir = String::from("/");
    let outer_root = &mut Box::new(Directory::new(&current_dir, None));
    for line in tree.lines() {
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
                    // println!("cd: {path}");
                    current_dir = match path {
                        "/" => "/".to_string(),
                        ".." => {
                            if let Some(dir) = root.find_dir(&current_dir) {
                                if let Some(parent) = &dir.parent {
                                    parent.clone()
                                } else {
                                    // new_parent = "".to_string();
                                    panic!("no parent path");
                                }
                            } else {
                                // println!("{:#?}", root);
                                // panic!("cannot find match for {current_dir:?}");
                                // can't go above "/"
                                "/".to_string()
                            }
                        }
                        _ => {
                            // println!("child dir: {}", current_dir);
                            if current_dir == "/" {
                                root.add_item(
                                    Item::Dir(Box::new(Directory::new(
                                        &path.to_string(),
                                        Some("/".to_string()),
                                    ))),
                                    0,
                                );
                            } else {
                                if let Some(dir) = root.find_dir(&current_dir) {
                                    dir.add_item(
                                        Item::Dir(Box::new(Directory::new(
                                            &path.to_string(),
                                            Some(dir.path.clone()),
                                        ))),
                                        0,
                                    );
                                }
                            }
                            path.to_string()
                        }
                    };
                }
                [Some("ls"), _] => {
                    // println!("ls");
                }
                _ => panic!("unknown instruction"),
            }
        } else {
            // result
            match parts[..] {
                ["dir", _] => {
                    // do nothing as we'll add them when cd happens
                    // println!("directory: {dir_name}")
                }
                [filesize, filename] => {
                    // println!("filesize: {filesize}, filename: {filename}");
                    let file_size = filesize.parse::<i32>().unwrap();
                    if current_dir == "/" {
                        root.add_item(Item::File((filename.to_string(), file_size)), file_size);
                    } else if let Some(dir) = root.find_dir(&current_dir) {
                        dir.add_item(Item::File((filename.to_string(), file_size)), file_size);
                    } else {
                        panic!("can't find");
                    }
                }
                _ => panic!("unknown result"),
            }
        }
        *outer_root = root;
    }

    outer_root.to_owned()
}

fn total_walker(node: &mut Box<Directory>, totals: &mut Vec<(String, i32)>) -> i32 {
    let dirs: &mut Vec<&Item> = &mut node
        .contents
        .iter()
        .filter(|item| match item {
            Item::Dir(_) => true,
            Item::File(_) => false,
        })
        .collect();
    if dirs.len() > 0 {
        let sub_dirs_totals: Vec<i32> = dirs
            .iter_mut()
            .map(|dir| {
                if let Item::Dir(mut dir) = dir.to_owned() {
                    total_walker(&mut dir, totals)
                } else {
                    panic!("ded");
                }
            })
            .collect();
        let sub_dir_total = sub_dirs_totals
            .iter()
            .copied()
            .reduce(|acc, curr| acc + curr)
            .unwrap();
        let node_total = sub_dir_total + node.file_size;
        node.total_size = node_total;
        totals.push((node.path.clone(), node_total));
        // println!("dir {}, total {}", node.path, node_total);
        node_total
    } else {
        // if node.file_size == 0 {
        //     println!("{:#?}", node);
        //     panic!("wot");
        // }
        totals.push((node.path.clone(), node.file_size));
        // println!("dir {}, total {}", node.path, node.file_size);
        node.file_size
    }
}

fn walk_for_dir_totals(node: &mut Box<Directory>) -> Vec<i32> {
    let mut totals = Vec::new();
    total_walker(node, &mut totals);
    totals.iter().map(|(_, total)| total.clone()).collect()
}
