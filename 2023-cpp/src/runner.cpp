#include <chrono>
#include <filesystem>
#include <fstream>
#include <iostream>
#include <stdexcept>
#include <string>
#include <vector>

#include "logging.tcc"
#include "runner.hpp"

using namespace std;

using Clock = std::chrono::steady_clock;
using Second = std::chrono::duration<double, std::ratio<1>>;

vector<string> read_file_path(const filesystem::path& path)
{
    ifstream input_stream(path.c_str());

    if (!input_stream) {
        cerr << "Can't open input file!" << endl;
        cerr << path.c_str() << endl;
    }

    vector<string> text;
    string line;
    while (getline(input_stream, line)) {
        text.push_back(line);
    }
    return text;
}

vector<string> RunnerBase::get_input_lines(filesystem::path data_path)
{
    const auto target_path = filesystem::path(path) += data_path;
    return read_file_path(target_path);
}

void RunnerBase::run_input_part_1(const char* target_data_path)
{
    if (skippable) {
        print_line("Skipping Day " + day_name + " part 2");
        return;
    }
    run_input_part_1(target_data_path, get_part_1_expected());
}

void RunnerBase::run_input_part_1(const char* target_data_path, int expected)
{
    print_line("Day " + day_name + ": Part 1");
    auto data = get_input_lines(filesystem::path(target_data_path));
    std::chrono::time_point<Clock> m_beg { Clock::now() };
    auto result = get_part_1_result(&data);
    auto time = std::chrono::duration_cast<Second>(Clock::now() - m_beg).count();
    print_line("     time taken: " + to_string(time));
    runner_result_expected(result, expected);
}

void RunnerBase::run_input_part_2(const char* target_data_path)
{
    if (skippable) {
        print_line("Skipping Day " + day_name + " part 2");
        return;
    }
    run_input_part_2(target_data_path, get_part_2_expected());
}
void RunnerBase::run_input_part_2(const char* target_data_path, int expected)
{
    print_line("Day " + day_name + ": Part 2");
    auto data = get_input_lines(filesystem::path(target_data_path));
    std::chrono::time_point<Clock> m_beg { Clock::now() };
    auto result = get_part_2_result(&data);
    auto time = std::chrono::duration_cast<Second>(Clock::now() - m_beg).count();
    print_line("     time taken: " + to_string(time));
    runner_result_expected(result, expected);
}

void RunnerBase::runner_result_expected(int result, int expected)
{
    cout << "  Result: ";
    cout << result << endl;
    if (result != expected) {
        print_error("   Expected: " + to_string(expected));
        print_error("   Failed");
        exit(1);
    }
    cout << "Correct!" << endl;
    cout << endl;
}

int RunnerBase::get_part_1_expected() { return part_1_expected; }
int RunnerBase::get_part_2_expected() { return part_2_expected; }

int RunnerBase::get_part_1_result(problem_lines) { throw std::logic_error("Please implement for each given day"); }
int RunnerBase::get_part_2_result(problem_lines) { throw std::logic_error("Please implement for each given day"); }