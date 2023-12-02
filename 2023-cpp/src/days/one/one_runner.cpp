#include "one_runner.hpp"

#include "one.hpp"
#include <utility>

using namespace std;

One::One(filesystem::path targetPath, int part_1, int part_2)
    : RunnerBase(std::move(targetPath), part_1, part_2) {};

void One::part_1(problem_lines lines, int expected)
{
    OneImpl one;
    auto result = one.part_1(lines);
    cout << "Result: ";
    cout << result << endl;
    if (result != expected) {
        print_error("Failed");
        exit(1);
    }
    cout << "Correct!" << endl;
    cout << endl;
};

void One::part_2(problem_lines lines, int expected)
{
    OneImpl one;
    auto result = one.part_2(lines);
    cout << "Result: ";
    cout << result << endl;
    if (result != expected) {
        print_error("Failed");
        exit(1);
    }
    cout << "Correct!" << endl;
    cout << endl;
};
