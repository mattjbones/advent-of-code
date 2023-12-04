#include <algorithm>
#include <array>
#include <boost/algorithm/string.hpp>
#include <cstring>
#include <iostream>
#include <map>
#include <numeric>
#include <regex>
#include <string>
#include <vector>

#include "../../logging.tcc"
#include "../../utils.tcc"

#include "three.hpp"

using namespace std;

struct Coord {
    int x;
    int y;
};

Coord operator+(Coord const& lhs, Coord const& rhs) { return Coord { lhs.x + rhs.x, lhs.y + rhs.y }; };
Coord operator-(Coord const& lhs, Coord const& rhs) { return Coord { lhs.x - rhs.x, lhs.y - rhs.y }; };
bool eq(Coord const& lhs, Coord const& rhs) { return lhs.x == rhs.x && lhs.y == rhs.y; };
void print_coord(Coord const& coord) { print_line("x: " + to_string(coord.x) + ", y: " + to_string(coord.y)); }
bool contains_coord(vector<Coord>& coords, const Coord& target)
{
    return std::any_of(coords.begin(), coords.end(), [&target](Coord& coord) { return eq(coord, target); });
}

// vector<vector<char>> generate_matrix(problem_lines ptr_lines)
// {

//     vector<vector<char>> schematic;

//     std::for_each(ptr_lines->begin(), ptr_lines->end(), [&schematic](string& line) mutable {
//         vector<char> line_chars;
//         for (const char character : line) {
//             line_chars.push_back(character);
//         }
//         schematic.push_back(line_chars);
//     });

//     // std::for_each(schematic.begin(), schematic.end(), [](vector<char> row) { print_lines(&row); });
//     return schematic;
// }

const std::array<int, 10> CHAR_NUMBERS = { '0', '1', '2', '3', '4', '5', '6', '7', '8', '9' };
const auto IGNORE = ".";

vector<int> walk_input_for_symbols(problem_lines ptr_lines, bool debug)
{
    vector<int> results;
    for (int line_no = 0; line_no < ptr_lines->size(); line_no++) {
        auto line = (*ptr_lines)[line_no];
        // walk until find non-number
        for (int char_no = 0; char_no < line.size(); char_no++) {
            auto character = line.at(char_no);
            if (debug) {
                cout << character;
                cout << " ";
            }
            if (character != *IGNORE && !contains(CHAR_NUMBERS, character)) {
                // cout << "SYMBOL";
                const Coord here = Coord { char_no, line_no };

                // cardinals
                auto left = char_no > 0 ? Coord { char_no - 1, line_no } : here;
                auto right = char_no < line.size() - 1 ? Coord { char_no + 1, line_no } : here;
                auto up = line_no > 0 ? Coord { char_no, line_no - 1 } : here;
                auto down = line_no < ptr_lines->size() - 1 ? Coord { char_no, line_no + 1 } : here;

                // diags
                auto up_left = Coord { left.x, up.y };
                auto up_right = Coord { right.x, up.y };
                auto down_right = Coord { right.x, down.y };
                auto down_left = Coord { left.x, down.y };

                const vector<Coord> coords = { left, right, up, down, up_left, up_right, down_left, down_right };
                // std::for_each(coords.begin(), coords.end(), [](Coord coord) { print_coord(coord); });

                // walk up / down / left / right / diag to find a number
                vector<Coord> visited = { here };
                for (const Coord coord : coords) {
                    if (contains_coord(visited, coord)) {
                        continue;
                    }
                    auto possible_number = (*ptr_lines)[coord.y].at(coord.x);
                    if (possible_number != *IGNORE && contains(CHAR_NUMBERS, possible_number)) {
                        // cout << "Start of number: ";
                        // cout << possible_number;
                        vector<char> parts;
                        parts.push_back(possible_number);

                        // check forward
                        // add to array from back
                        int forwards = coord.x + 1;
                        auto next_letter = (*ptr_lines)[coord.y].at(forwards);
                        while (contains(CHAR_NUMBERS, next_letter)) {
                            parts.push_back(next_letter);
                            visited.push_back(Coord { forwards, coord.y });
                            ++forwards;
                            if (forwards < line.size()) {
                                next_letter = (*ptr_lines)[coord.y].at(forwards);
                            } else {
                                break;
                            }
                        }

                        // check backwards
                        // add to array from front
                        int backwards = coord.x - 1;
                        next_letter = (*ptr_lines)[coord.y].at(backwards);
                        while (contains(CHAR_NUMBERS, next_letter)) {
                            parts.insert(parts.begin(), next_letter);
                            visited.push_back(Coord { backwards, coord.y });
                            --backwards;
                            if (backwards >= 0) {
                                next_letter = (*ptr_lines)[coord.y].at(backwards);
                            } else {
                                break;
                            }
                        }

                        string number;
                        std::for_each(parts.begin(), parts.end(), [&number](char& part) { number += part; });
                        // print_line("Number: " + number);
                        const int int_number = std::stoi(number);
                        results.push_back(int_number);
                    }
                }
            }
        }
        if (debug) {
            cout << endl;
        }
    }

    // print_lines(&results);
    return results;
}

int ThreeImpl::part_1(problem_lines ptr_lines)
{
    // auto schematic = generate_matrix(ptr_lines);
    auto numbers = walk_input_for_symbols(ptr_lines, false);
    return std::accumulate(numbers.begin(), numbers.end(), 0);
}

const auto GEAR_CHAR = "*";
int ThreeImpl::part_2(problem_lines ptr_lines)
{
    auto debug = false;
    vector<int> results;
    for (int line_no = 0; line_no < ptr_lines->size(); line_no++) {
        auto line = (*ptr_lines)[line_no];
        // walk until find non-number
        for (int char_no = 0; char_no < line.size(); char_no++) {
            auto character = line.at(char_no);
            if (debug) {
                cout << character;
                cout << " ";
            }
            if (character == *GEAR_CHAR) {
                // cout << "SYMBOL";
                const Coord here = Coord { char_no, line_no };

                // cardinals
                auto left = char_no > 0 ? Coord { char_no - 1, line_no } : here;
                auto right = char_no < line.size() - 1 ? Coord { char_no + 1, line_no } : here;
                auto up = line_no > 0 ? Coord { char_no, line_no - 1 } : here;
                auto down = line_no < ptr_lines->size() - 1 ? Coord { char_no, line_no + 1 } : here;

                // diags
                auto up_left = Coord { left.x, up.y };
                auto up_right = Coord { right.x, up.y };
                auto down_right = Coord { right.x, down.y };
                auto down_left = Coord { left.x, down.y };

                const vector<Coord> coords = { left, right, up, down, up_left, up_right, down_left, down_right };
                // std::for_each(coords.begin(), coords.end(), [](Coord coord) { print_coord(coord); });

                // walk up / down / left / right / diag to find a number
                vector<int> possible_numbers;
                vector<Coord> visited = { here };
                for (const Coord coord : coords) {
                    if (contains_coord(visited, coord)) {
                        continue;
                    }
                    auto possible_number = (*ptr_lines)[coord.y].at(coord.x);
                    if (possible_number != *IGNORE && contains(CHAR_NUMBERS, possible_number)) {
                        // cout << "Start of number: ";
                        // cout << possible_number;
                        vector<char> parts;
                        parts.push_back(possible_number);

                        // check forward
                        // add to array from back
                        int forwards = coord.x + 1;
                        auto next_letter = (*ptr_lines)[coord.y].at(forwards);
                        while (contains(CHAR_NUMBERS, next_letter)) {
                            parts.push_back(next_letter);
                            visited.push_back(Coord { forwards, coord.y });
                            ++forwards;
                            if (forwards < line.size()) {
                                next_letter = (*ptr_lines)[coord.y].at(forwards);
                            } else {
                                break;
                            }
                        }

                        // check backwards
                        // add to array from front
                        int backwards = coord.x - 1;
                        next_letter = (*ptr_lines)[coord.y].at(backwards);
                        while (contains(CHAR_NUMBERS, next_letter)) {
                            parts.insert(parts.begin(), next_letter);
                            visited.push_back(Coord { backwards, coord.y });
                            --backwards;
                            if (backwards >= 0) {
                                next_letter = (*ptr_lines)[coord.y].at(backwards);
                            } else {
                                break;
                            }
                        }

                        string number;
                        std::for_each(parts.begin(), parts.end(), [&number](char& part) { number += part; });
                        // print_line("Number: " + number);
                        const int int_number = std::stoi(number);
                        // results.push_back(int_number);
                        possible_numbers.push_back(int_number);
                    }
                }

                if (possible_numbers.size() == 2) {
                    results.push_back((possible_numbers[0] * possible_numbers[1]));
                }
            }
        }
        if (debug) {
            cout << endl;
        }
    }

    // print_lines(&results);
    return std::accumulate(results.begin(), results.end(), 0);
    ;
}