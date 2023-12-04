#include <benchmark/benchmark.h>
#include <filesystem>
#include <set>
#include <string>
#include <vector>

#include "../src/days/one/one_runner.hpp"
#include "../src/runner.hpp"

using namespace std;

vector<string> setup()
{
    One day_one(filesystem::path("../src/days/one/"), 54331, 54518);
    const filesystem::path data_path = filesystem::path(path) += filesystem::path("data/input");
    print_line("Data Path: ");
    print_line(data_path.c_str());
    vector<string> data = read_file_path(data_path);
    return data;
}

void run_part_1(vector<string>& data) { day_one.part_1(&data, day_one.part_1_expected); }

static void BM_Day1_Part1(benchmark::State& state)
{
    print_line("Benchy Day 1 - Part 1");
    // Perform setup here
    auto data = setup();

    for (auto _ : state) {
        // This code gets timed
        run_part_1(data);
    }
};

static void BM_Day1_Part2(benchmark::State& state)
{

    print_line("Benchy Day 1 - Part 2");
    // Perform setup here
    One day_one(filesystem::path("../src/days/one/"), 54331, 54518);
    vector<string> data = setup(day_one.path);

    for (auto _ : state) {
        // This code gets timed
        // day_one.part_2(&data, day_one.part_2_expected);
    }
};

// Register the function as a benchmark
BENCHMARK(BM_Day1_Part1);
BENCHMARK(BM_Day1_Part2);
// Run the benchmark
BENCHMARK_MAIN();