#include "two_runner.hpp"
#include "two.hpp"

using namespace std;

Two::Two(filesystem::path targetPath, int part_1, int part_2, const char* day)
    : RunnerBase(std::move(targetPath), part_1, part_2, day, true) {};

int Two::get_part_1_result(problem_lines ptr_lines)
{
    TwoImpl two;
    return two.part_1(ptr_lines);
}

int Two::get_part_2_result(problem_lines ptr_lines)
{
    TwoImpl two;
    return two.part_2(ptr_lines);
}
