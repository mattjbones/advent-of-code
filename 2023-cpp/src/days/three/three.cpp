#include <algorithm>
#include <array>
#include <boost/algorithm/string.hpp>
#include <map>
#include <numeric>
#include <regex>
#include <string>
#include <vector>

#include "../../logging.tcc"

#include "three.hpp"

using namespace std;

int ThreeImpl::part_1(problem_lines ptr_lines)
{
    vector<vector<char>> schematic;

    std::for_each(ptr_lines->begin(), ptr_lines->end(), [schematic](string& line) mutable {
        vector<char> line_chars;
        for (const char character : line) {
            line_chars.push_back(character);
        }
        schematic.push_back(line_chars);
    });

    // std::for_each(schematic.begin(), schematic.end(), [](vector<char> row) { print_lines(row); });

    return 0;
}

int ThreeImpl::part_2(problem_lines ptr_lines) { return 0; }