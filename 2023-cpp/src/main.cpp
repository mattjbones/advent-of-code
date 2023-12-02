#include <algorithm>
#include <iostream>

#include "logging.hpp"
#include "runner.hpp"

#include "days/one/one_runner.hpp"
#include "days/two/two_runner.hpp"

using namespace std;

void run_input(RunnerBase* day, string path_part = "data/input");
void run_input_part_1(RunnerBase* day, int expected, string path_part = "data/input");
void run_input_part_2(RunnerBase* day, int expected, string path_part = "data/input");

int main()
{
    cout << "Advent of Code 2023!" << endl;
    cout << endl;

    One day_one(filesystem::path("./src/days/one/"), 54331, 54518);
    Two day_two(filesystem::path("./src/days/two/"), 0, 0);

    vector<RunnerBase*> days = { &day_one, &day_two };

    // run samples
    print_line("Running sample data");
    run_input_part_1(&day_one, 142, "data/sample_1");
    run_input_part_2(&day_one, 281, "data/sample_2");

    run_input_part_1(&day_two, 8, "data/sample_1");

    // run input
    print_line("Running input data");
    std::for_each(days.begin(), days.end(), [](RunnerBase* day) { run_input(day); });
}

void run_input_part_1(RunnerBase* day, int expected, string path_part)
{
    filesystem::path path = filesystem::path(day->path) += filesystem::path(path_part);
    vector<string> data = read_file_path(path);
    day->part_1(&data, expected);
}

void run_input_part_2(RunnerBase* day, int expected, string path_part)
{
    filesystem::path path = filesystem::path(day->path) += filesystem::path(path_part);
    vector<string> data = read_file_path(path);
    day->part_2(&data, expected);
}

void run_input(RunnerBase* day, string path_part)
{
    filesystem::path path = filesystem::path(day->path) += filesystem::path(path_part);
    vector<string> data = read_file_path(path);
    day->part_1(&data, day->part_1_expected);
    day->part_2(&data, day->part_2_expected);
}
