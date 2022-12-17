pub trait Runner<T>
where
    T: std::fmt::Display,
{
    fn run_all(&self, sample: &str, input: &str) {
        {
            println!();
            println!("Part 1 - Sample");
            let (result, expected) = self.part_1_sample(sample);
            println!(" Result {result}");
            println!(" Expected {expected}");
        }

        {
            println!();
            println!("Part 1 - Input");
            let (result, expected) = self.part_1_input(input);
            println!(" Result {result}");
            println!(" Expected {expected}");
        }

        {
            println!();
            println!("Part 2 - Sample");
            let (result, expected) = self.part_2_sample(sample);
            println!(" Result {result}");
            println!(" Expected {expected}");
        }

        {
            println!();
            println!("Part 2 - Input");
            let (result, expected) = self.part_2_input(input);
            println!(" Result {result}");
            println!(" Expected {expected}");
        }
    }

    fn part_1_sample(&self, sample: &str) -> (T, T);
    fn part_1_input(&self, input: &str) -> (T, T);
    fn part_2_sample(&self, sample: &str) -> (T, T);
    fn part_2_input(&self, input: &str) -> (T, T);
}
