#include <benchmark/benchmark.h>
#include <filesystem>
#include <set>
#include <string>
#include <vector>

#include "../src/days/one/one.cpp"
#include "../src/days/one/one_runner.cpp"
#include "../src/logging.tcc"
#include "../src/runner.cpp"

using namespace std;

static void BM_Day1_Part1(benchmark::State& state)
{
    print_line("Benchy Day 1 - Part 1");
    // Perform setup here
    One day_one(filesystem::path("src/days/one/"), 54331, 54518, "One");

    for (auto _ : state) {
        // This code gets timed
        day_one.run_input_part_1("data/input");
    }
};

static void BM_Day1_Part2(benchmark::State& state)
{

    print_line("Benchy Day 1 - Part 2");
    // Perform setup here
    One day_one(filesystem::path("src/days/one/"), 54331, 54518, "One");

    for (auto _ : state) {
        // This code gets timed
        day_one.run_input_part_2("data/input");
    }
};

// Register the function as a benchmark
BENCHMARK(BM_Day1_Part1);
BENCHMARK(BM_Day1_Part2);
// Run the benchmark
BENCHMARK_MAIN();