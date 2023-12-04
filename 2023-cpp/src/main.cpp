#include <algorithm>
#include <iostream>
#include <string>

#include "logging.tcc"
#include "runner.hpp"

#include "days/four/four.hpp"
#include "days/one/one_runner.hpp"
#include "days/three/three_runner.hpp"
#include "days/two/two_runner.hpp"

using namespace std;

void run_input(RunnerBase* day, string path_part = "data/input");
void run_input_part_1(RunnerBase* day, int expected, string path_part = "data/input");
void run_input_part_2(RunnerBase* day, int expected, string path_part = "data/input");
void run_input_part_1(RunnerBase* day, int expected, vector<string> data);
void run_input_part_2(RunnerBase* day, int expected, vector<string> data);

int main()
{
    print_line("Advent of Code 2023!");

    One day_one(filesystem::path("./src/days/one/"), 54331, 54518, "One");
    Two day_two(filesystem::path("./src/days/two/"), 2720, 71535, "Two");
    Three day_three(filesystem::path("./src/days/three/"), 544664, 84495585, "Three");
    Four day_four(32001, -1);

    vector<RunnerBase*> days = { &day_one, &day_two, &day_three, &day_four };

    // run samples
    print_line("Running sample data");

    day_four.run_input_part_1("data/sample_1", 13);
    day_four.run_input_part_2("data/sample_1", 30);

    day_three.run_input_part_1("data/sample_1", 4361);
    day_three.run_input_part_2("data/sample_1", 467835);

    day_two.run_input_part_1("data/sample_1", 8);
    day_two.run_input_part_2("data/sample_1", 2286);

    day_one.run_input_part_1("data/sample_1", 142);
    day_one.run_input_part_2("data/sample_2", 281);

    // run input
    print_line("=== Running input data ===");
    std::for_each(days.begin(), days.end(), [](RunnerBase* day) mutable {
        day->run_input_part_1();
        day->run_input_part_2();
    });

    print_line("\tEverything is correct! 🔥\n\n");
}