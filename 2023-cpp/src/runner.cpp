#include <filesystem>
#include <fstream>
#include <iostream>
#include <stdexcept>
#include <string>
#include <vector>

#include "logging.tcc"
#include "runner.hpp"

using namespace std;

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
    run_input_part_1(target_data_path, get_part_1_expected());
}

void RunnerBase::run_input_part_1(const char* target_data_path, int expected)
{
    print_line("Part 1");
    auto data = get_input_lines(filesystem::path(target_data_path));
    auto result = get_part_1_result(&data);
    runner_result_expected(result, expected);
}

void RunnerBase::run_input_part_2(const char* target_data_path)
{
    run_input_part_2(target_data_path, get_part_2_expected());
}
void RunnerBase::run_input_part_2(const char* target_data_path, int expected)
{
    print_line("Part 2");
    auto data = get_input_lines(filesystem::path(target_data_path));
    auto result = get_part_2_result(&data);
    runner_result_expected(result, expected);
}

void RunnerBase::runner_result_expected(int result, int expected)
{
    cout << "Result: ";
    cout << result << endl;
    if (result != expected) {
        print_error("Failed");
        exit(1);
    }
    cout << "Correct!" << endl;
    cout << endl;
}

int RunnerBase::get_part_1_expected() { return part_1_expected; }
int RunnerBase::get_part_2_expected() { return part_2_expected; }

int RunnerBase::get_part_1_result(problem_lines) { throw std::logic_error("Not implemented"); }
int RunnerBase::get_part_2_result(problem_lines) { throw std::logic_error("Not implemented"); }