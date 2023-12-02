#include <algorithm>
#include <iostream>
#include <string>

#include "logging.hpp"
#include "runner.hpp"

#include "days/one/one_runner.hpp"
#include "days/two/two_runner.hpp"

using namespace std;

void run_input(RunnerBase* day, string path_part = "data/input");
void run_input_part_1(RunnerBase* day, int expected, string path_part = "data/input");
void run_input_part_2(RunnerBase* day, int expected, string path_part = "data/input");
void run_input_part_1(RunnerBase* day, int expected, vector<string> data);
void run_input_part_2(RunnerBase* day, int expected, vector<string> data);

int main()
{
    cout << "Advent of Code 2023!" << endl;
    cout << endl;

    One day_one(filesystem::path("./src/days/one/"), 54331, 54518);
    Two day_two(filesystem::path("./src/days/two/"), 2720, 71535);

    vector<RunnerBase*> days = { &day_one, &day_two };

    // run samples
    print_line("Running sample data");
    run_input_part_1(&day_one, 142, "data/sample_1");
    run_input_part_2(&day_one, 281, "data/sample_2");

    run_input_part_1(&day_two, 8, "data/sample_1");
    run_input_part_2(&day_two, 2286, "data/sample_1");

    // run input
    print_line("Running input data");
    int day_number = 1;
    std::for_each(days.begin(), days.end(), [day_number](RunnerBase* day) mutable {
        print_line("- Day " + to_string(day_number) + " -");
        run_input(day);
        ++day_number;
    });
}

void run_input_part_1(RunnerBase* day, int expected, vector<string> data) { day->part_1(&data, expected); }

void run_input_part_2(RunnerBase* day, int expected, vector<string> data) { day->part_2(&data, expected); }

void run_input_part_1(RunnerBase* day, int expected, string path_part)
{
    const filesystem::path path = filesystem::path(day->path) += filesystem::path(path_part);
    vector<string> data = read_file_path(path);
    print_line("Part 1");
    day->part_1(&data, expected);
}

void run_input_part_2(RunnerBase* day, int expected, string path_part)
{
    const filesystem::path path = filesystem::path(day->path) += filesystem::path(path_part);
    vector<string> data = read_file_path(path);
    print_line("Part 2");
    day->part_2(&data, expected);
}

void run_input(RunnerBase* day, string path_part)
{
    const filesystem::path path = filesystem::path(day->path) += filesystem::path(path_part);
    vector<string> data = read_file_path(path);

    print_line("Part 1");
    day->part_1(&data, day->part_1_expected);

    print_line("Part 2");
    day->part_2(&data, day->part_2_expected);
}
