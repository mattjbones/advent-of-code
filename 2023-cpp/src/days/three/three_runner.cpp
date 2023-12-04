#include "three_runner.hpp"
#include "three.hpp"
#include <algorithm>

using namespace std;

Three::Three(filesystem::path targetPath, int part_1, int part_2, const char* day)
    : RunnerBase(std::move(targetPath), part_1, part_2, day) {};

int Three::get_part_1_result(problem_lines lines)
{
    ThreeImpl three;
    return three.part_1(lines);
}

int Three::get_part_2_result(problem_lines lines)
{
    ThreeImpl three;
    return three.part_2(lines);
}
