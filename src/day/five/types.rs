use std::fmt::Debug;
use std::io::{Error, ErrorKind};
use std::result::Result;

#[derive(Debug)]
pub struct Stack(Vec<Option<Container>>);

impl Stack {
    pub fn push(&mut self, container: Option<Container>) {
        self.0.push(container);
    }

    fn first_some_container(&self) -> usize {
        let index = self
            .0
            .iter()
            .rev()
            .position(|container| container.is_some())
            .unwrap_or(0);
        self.0.len() - index - 1
    }

    pub fn take(&mut self) -> Option<Container> {
        self.take_many(1).pop().unwrap()
    }

    pub fn take_many(&mut self, count: usize) -> Vec<Option<Container>> {
        let index = self.first_some_container();
        let mut taken = Vec::new();
        for counter in 0..count {
            taken.push(self.0.remove(index - counter));
        }
        taken.reverse();
        taken
    }

    pub fn put(&mut self, container: Option<Container>) {
        self.put_many(Vec::from([container]));
        // let empty_space = self.0.iter().position(|item| item.is_none());
        // if let Some(empty_index) = empty_space {
        //     self.0.insert(empty_index, container);
        // } else {
        //     self.0.push(container);
        // }
    }

    pub fn put_many(&mut self, mut containers: Vec<Option<Container>>) {
        let empty_space = self.0.iter().position(|item| item.is_none());

        let mut count = 0;
        while containers.len() > 0 {
            let container = containers.remove(0);
            if let Some(empty_index) = empty_space {
                self.0.insert(empty_index + count, container);
                count += 1;
            } else {
                self.0.push(container);
            }
        }
    }
}

#[derive(Debug)]
pub struct Container(String);

impl TryFrom<&[char]> for Container {
    type Error = Error;
    fn try_from(range: &[char]) -> Result<Self, Self::Error> {
        let mut string = String::new();
        for char in range {
            if !char.is_whitespace() && !char.is_numeric() {
                string.push(*char);
            }
        }
        if string.len() > 0 {
            Ok(Self(string))
        } else {
            Err(Error::from(ErrorKind::NotFound))
        }
    }
}

pub struct Dock {
    stacks: Vec<Stack>,
    size: usize,
    next: usize,
}

impl Dock {
    pub fn new(size: usize) -> Self {
        Dock {
            stacks: Vec::new(),
            size,
            next: 0,
        }
    }

    pub fn push_next(&mut self, container: Option<Container>) {
        if self.stacks.len() != self.size {
            self.stacks.push(Stack(Vec::from([container])));
        } else {
            self.stacks[self.next].push(container);
            self.next += 1;
            if self.next >= self.size {
                self.next = 0;
            }
        }
    }

    pub fn flip(&mut self) {
        self.stacks.iter_mut().for_each(|stack| stack.0.reverse())
    }

    pub fn move_containers_9000(&mut self, from: usize, to: usize, count: usize) {
        for _ in 0..count {
            let moving = self.stacks[from - 1].take();
            self.stacks[to - 1].put(moving);
        }
    }

    pub fn move_containers_9001(&mut self, from: usize, to: usize, count: usize) {
        let moving = self.stacks[from - 1].take_many(count);
        self.stacks[to - 1].put_many(moving);
    }

    pub fn tops(&self) -> String {
        let mut tops = String::new();
        for stack in self.stacks.iter() {
            let top_container = stack
                .0
                .iter()
                .rev()
                .find(|container| container.is_some())
                .unwrap()
                .as_ref()
                .unwrap();
            tops = format!(
                "{}{}",
                tops,
                top_container.0.replace("[", "").replace("]", "")
            );
        }
        tops
    }

    pub fn prune(&mut self) {
        self.stacks.iter_mut().for_each(|stack| {
            stack.0.pop();
        });
    }
}

impl Debug for Dock {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        // let stacks = &self.stacks;
        // let mut longest = 0;
        // stacks.iter().for_each(|stack| {
        //     if stack.0.len() > longest {
        //         longest = stack.0.len();
        //     }
        // });

        // let mut line = String::new();
        // for index in 0..longest {
        //     for stack in stacks {
        //         println!("{index} len{}", stack.0.len());

        //         if stack.0.len() < index {
        //             let container = stack.0[index].as_ref();
        //             println!("{:?}", container);
        //             let string = container.map_or("   ", |container| &container.0);
        //             line.push_str(string);
        //         } else {
        //             line.push_str("   ");
        //         }
        //     }
        // }

        // f.write_str(line.as_str())

        f.debug_struct("Dock")
            .field("stacks", &self.stacks)
            .field("size", &self.size)
            .field("next", &self.next)
            .finish()
    }
}
