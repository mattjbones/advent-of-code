pub fn run() {
    println!("Day 13");
    {
        println!("Part 1");
        let result = part_1(include_str!("sample"));
        println!(" Result {result}");
        println!(" Expected 13");
    }
}

fn part_1(input: &str) -> usize {
    let pairs = input.split("\n\n").collect::<Vec<&str>>();
    println!("pairs: {:?}", pairs);
    0
}
