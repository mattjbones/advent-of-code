use crate::runner::Runner;
mod app;
use app::*;

pub struct Seventeen {}
impl Runner<usize> for Seventeen {
    fn part_1_sample(&self, sample: &str) -> (usize, usize) {
        let mut chamber = Chamber::new(7, sample.chars().collect::<Vec<char>>(), 2022);
        let result = chamber.simulate(false, true, false);
        (result, 3068)
    }

    fn part_1_input(&self, input: &str) -> (usize, usize) {
        let mut chamber = Chamber::new(7, input.chars().collect::<Vec<char>>(), 2022);
        let result = chamber.simulate(false, true, false);
        (result, 3124)
    }

    fn part_2_sample(&self, sample: &str) -> (usize, usize) {
        let mut chamber = Chamber::new(7, sample.chars().collect::<Vec<char>>(), 1_000_000_000_000);
        let result = chamber.simulate(false, false, true);
        (result, 1514285714288)
    }

    fn part_2_input(&self, input: &str) -> (usize, usize) {
        let mut chamber = Chamber::new(7, input.chars().collect::<Vec<char>>(), 1_000_000_000_000);
        let result = chamber.simulate(false, false, true);
        (result, 0)
    }
}
