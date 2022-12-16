pub trait Runner {
    fn run_all(&self, sample: &str, input: &str) {
        self.part_1_sample(sample);
        self.part_1_input(input);
        self.part_2_sample(sample);
        self.part_2_input(input);
    }
    fn part_1_sample(&self, sample: &str);
    fn part_1_input(&self, input: &str);
    fn part_2_sample(&self, sample: &str);
    fn part_2_input(&self, input: &str);
}
