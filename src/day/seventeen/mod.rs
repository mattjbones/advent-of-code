use crate::runner::Runner;
mod app;
use app::*;

pub struct Seventeen {}
impl Runner<usize> for Seventeen {
    fn part_1_sample(&self, sample: &str) -> (usize, usize) {
        let chamber = Chamber::new(7, sample.chars().collect::<Vec<char>>(), 2022);
        chamber.rocks.iter().for_each(|rock| {
            println!("{:?}", rock.rock_type);
            println!("{:?}", PrettyPositions(Box::new(rock.coords.clone()), 0));
        });
        let result = chamber.simulate();
        (result, 3068)
    }

    fn part_1_input(&self, _: &str) -> (usize, usize) {
        todo!();
    }

    fn part_2_sample(&self, _: &str) -> (usize, usize) {
        todo!();
    }

    fn part_2_input(&self, _: &str) -> (usize, usize) {
        todo!();
    }
}
