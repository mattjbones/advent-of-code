use crate::Runner;

pub struct Twenty {}
impl Runner<usize> for Twenty {
    fn part_1_sample(&self, _: &str) -> (usize, usize) {
        let result = 1;
        (result, 0)
    }

    fn part_1_input(&self, _: &str) -> (usize, usize) {
        let result = 1;
        (result, 0)
    }

    fn part_2_sample(&self, _: &str) -> (usize, usize) {
        let result = 1;
        (result, 0)
    }

    fn part_2_input(&self, _: &str) -> (usize, usize) {
        let result = 1;
        (result, 0)
    }
}

struct WrappingIndex {
    length: isize,
}

impl WrappingIndex {
    fn new(_: usize, length: usize) -> Self {
        Self {
            length: length as isize,
        }
    }

    fn get_next_index_forward(&self, index: usize) -> usize {
        let next_index: isize = (index + 1).try_into().unwrap();
        if next_index > self.length {
            0
        } else {
            next_index
        };
        next_index as usize
    }

    fn get_next_index_backwards(&self, index: usize, reducer: usize) -> usize {
        let index = index as isize;
        let reducer = reducer as isize;
        let new_index = index - reducer;
        if self.length % new_index == 0 {
            // loops around many times
            new_index as usize
        } else {
            let next_index = if new_index == 0 {
                self.length - 1
            } else {
                (new_index - 1).try_into().unwrap()
            };
            next_index as usize
        }
    }
}

fn move_number(selected_number: isize, list: Vec<isize>) -> Vec<isize> {
    let mut working_list = list.clone();
    let is_forward = selected_number > 0;
    let start_index = working_list
        .iter()
        .position(|&no| no == selected_number)
        .unwrap();

    println!("working_list {:?}", working_list);
    println!("start_index {}", start_index);

    let rapping = WrappingIndex::new(start_index, working_list.len());

    let count = if is_forward {
        selected_number
    } else {
        selected_number * -1
    } as usize;

    for i in 0..count {
        let current_index = if is_forward {
            rapping.get_next_index_forward(start_index + i)
        } else {
            rapping.get_next_index_backwards(start_index, i)
        };

        let next_index = if is_forward {
            rapping.get_next_index_forward(current_index)
        } else {
            rapping.get_next_index_backwards(current_index, 1)
        };

        let no = working_list.remove(current_index);
        println!("working_list {:?}", working_list);
        println!("no {:?}", no);
        working_list.insert(next_index, no);
    }

    working_list
}

#[cfg(test)]
mod tests {
    // use super::*;

    // #[test]
    // fn test_move_number_sequence_pos() {
    //     let sequence = vec![4, 5, 6, 1, 7, 8, 9];
    //     let expected = vec![4, 5, 6, 7, 1, 8, 9];

    //     assert_eq!(expected, move_number(1, sequence));
    // }

    // #[test]
    // fn test_move_number_sequence_neg() {
    //     let sequence = vec![4, -2, 5, 6, 1, 7, 8, 9];
    //     let expected = vec![4, 5, 6, 1, 7, 8, -2, 9];
    //     assert_eq!(expected, move_number(-2, sequence));
    // }
}
