pub trait Runner {
    fn run_all(&self) {
        self.part_1_sample();
        self.part_1_input();
        self.part_2_sample();
        self.part_2_input();
    }
    fn part_1_sample(&self);
    fn part_1_input(&self);
    fn part_2_sample(&self);
    fn part_2_input(&self);
}
