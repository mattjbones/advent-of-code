#include "two_runner.hpp"
#include "two.hpp"

using namespace std;

Two::Two(filesystem::path targetPath, int part_1, int part_2)
    : RunnerBase(targetPath, part_1, part_2) {};

void Two::part_1(problem_lines lines, int expected)
{
    print_line("Day Two: Part 1");
    TwoImpl two;
    auto result = two.part_1(lines);
    cout << "Result: ";
    cout << result << endl;
    if (result != expected) {
        print_error("Failed");
        exit(1);
    }
    cout << "Correct!" << endl;
    cout << endl;
};

void Two::part_2(problem_lines lines, int expected)
{
    print_line("Day Two: Part 2");
    TwoImpl two;
    auto result = two.part_2(lines);
    cout << "Result: ";
    cout << result << endl;
    if (result != expected) {
        print_error("Failed");
        exit(1);
    }
    cout << "Correct!" << endl;
    cout << endl;
};
