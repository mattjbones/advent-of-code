#include <algorithm>
#include <iostream>
#include <string>

#include "logging.tcc"
#include "runner.hpp"

#include "days/eight/eight.hpp"
#include "days/five/five.hpp"
#include "days/four/four.hpp"
#include "days/one/one_runner.hpp"
#include "days/seven/seven.hpp"
#include "days/six/six.hpp"
#include "days/three/three_runner.hpp"
#include "days/two/two_runner.hpp"

using namespace std;

int main()
{
    print_line("Advent of Code 2023!");

    One day_one(filesystem::path("./src/days/one/"), 54331, 54518, "One");
    Two day_two(filesystem::path("./src/days/two/"), 2720, 71535, "Two");
    Three day_three(filesystem::path("./src/days/three/"), 544664, 84495585, "Three");
    Four day_four(32001, 5037841);
    Five day_five(650599855); // skipped
    Six day_six(2269432, 35865985);
    Seven day_seven(251927063, 255632664);
    Eight day_eight(16531);

    vector<RunnerBase*> days
        = { &day_one, &day_two, &day_three, &day_four, &day_five, &day_six, &day_seven, &day_eight };

    // run samples
    print_line("Running sample data");

    day_eight.run_input_part_1("data/sample", 6);
    // day_eight.run_input_part_2("data/sample", 6);

    day_seven.run_input_part_1("data/sample", 6440);
    day_seven.run_input_part_2("data/sample", 5905);

    day_six.run_input_part_1("data/sample", 288);
    day_six.run_input_part_2("data/sample", 71503);

    day_five.run_input_part_1("data/sample", 35);
    // TODO - properly
    //  day_five.run_input_part_2("data/sample", 46);

    day_four.run_input_part_1("data/sample_1", 13);
    day_four.run_input_part_2("data/sample_1", 30);

    day_three.run_input_part_1("data/sample_1", 4361);
    day_three.run_input_part_2("data/sample_1", 467835);

    // SHRUG
    // day_two.run_input_part_1("data/sample_1", 8);
    // day_two.run_input_part_2("data/sample_1", 2286);

    day_one.run_input_part_1("data/sample_1", 142);
    day_one.run_input_part_2("data/sample_2", 281);

    // run input
    print_line("=== Running input data ===");
    std::for_each(days.begin(), days.end(), [](RunnerBase* day) {
        day->run_input_part_1();
        day->run_input_part_2();
    });

    print_line("\tEverything is correct! 🔥\n\n");
}