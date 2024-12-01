#include <__tuple_dir/tuple_element.h>
#include <algorithm>
#include <filesystem>
#include <iterator>
#include <numeric>
#include <random>
#include <string>
#include <tuple>
#include <vector>

#include "ten.hpp"

Ten::Ten(int part_1, int part_2)
    : RunnerBase(filesystem::path("src/days/ten/"), part_1, part_2, "Ten") {};

const std::map<char, std::vector<char>> CONNECTION_MAP = {
    { '|', { 'F', 'L', '7', 'J' } },
    {
        '-',
        { 'F', 'L', '7', 'J' },
    },
    { 'L', { '-', '|' } },
    { 'F', { '-', '|' } },
    { '7', { '-', '|' } },
    { 'J', { '-', '|' } },
};

bool can_advance(char& current_position, char& next_position)
{
    std::vector<char> next_posibilities = CONNECTION_MAP.at(current_position);
    return contains(next_posibilities, next_position);
}

int Ten::get_part_1_result(problem_lines ptr_lines)
{
    print_lines(ptr_lines);

    // find S
    std::tuple<int, int> start_point;
    for (int i = 0; i < ptr_lines->size(); i++) {
        string line = ptr_lines->at(i);

        // const int col_index = index_of(line, 'S');
        auto col_index = index_of(line, 'S');
        print_line(col_index);

        if (col_index != -1) {
            // starting line
            start_point = { i, col_index };
            break;
        }
    }

    // iterate left / right
    // check cardinals
    // check if next last in seen array for other side (x2)
    std::vector<char> seen_left = { 'S' };
    std::vector<char> seen_right = { 'S' };

    auto is_end = false;
    auto breaker = 0;
    auto left = start_point;
    auto right = start_point;

    while (!is_end) {

        if (++breaker == 10) {
            print_line("Infinite loop?");
            exit(1);
        };
    }

    return 0;
}

int Ten::get_part_2_result(problem_lines ptr_lines) { return 0; }