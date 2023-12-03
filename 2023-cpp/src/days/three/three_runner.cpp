#include "three_runner.hpp"
#include "three.hpp"
#include <algorithm>

using namespace std;

Three::Three(filesystem::path targetPath, int part_1, int part_2)
    : RunnerBase(std::move(targetPath), part_1, part_2) {};

void Three::part_1(problem_lines lines, int expected)
{
    ThreeImpl three;
    auto result = three.part_1(lines);
    cout << "Result: ";
    cout << result << endl;
    if (result != expected) {
        print_error("Failed");
        exit(1);
    }
    cout << "Correct!" << endl;
    cout << endl;
};

void Three::part_2(problem_lines lines, int expected)
{
    ThreeImpl three;
    auto result = three.part_2(lines);
    cout << "Result: ";
    cout << result << endl;
    if (result != expected) {
        print_error("Failed");
        exit(1);
    }
    cout << "Correct!" << endl;
    cout << endl;
};
