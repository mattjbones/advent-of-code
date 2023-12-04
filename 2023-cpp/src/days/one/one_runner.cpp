#include "one_runner.hpp"

#include "one.hpp"
#include <utility>

using namespace std;

One::One(filesystem::path targetPath, int part_1, int part_2, const char* day)
    : RunnerBase(std::move(targetPath), part_1, part_2, day) {};

int One::get_part_1_result(problem_lines ptr_lines)
{
    OneImpl one;
    return one.part_1(ptr_lines);
}

int One::get_part_2_result(problem_lines ptr_lines)
{
    OneImpl one;
    return one.part_2(ptr_lines);
}
