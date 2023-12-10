

#include "nine.hpp"
#include <_types/_intmax_t.h>
#include <algorithm>
#include <boost/algorithm/string/classification.hpp>
#include <boost/algorithm/string/split.hpp>
#include <exception>
#include <vector>

using namespace std;

Nine::Nine(int part_1, int part_2)
    : RunnerBase(filesystem::path("src/days/nine/"), part_1, part_2, "Nine") {};

std::vector<std::vector<int>> Nine::parse_input(problem_lines ptr_lines)
{
    std::vector<std::vector<int>> lines;

    std::for_each(ptr_lines->begin(), ptr_lines->end(), [&lines](auto& line) {
        std::vector<int> history_sequence;

        std::vector<string> history_sequence_string;
        boost::split(history_sequence_string, line, boost::is_any_of(" "));

        std::transform(history_sequence_string.begin(), history_sequence_string.end(),
            std::back_inserter(history_sequence), [](const std::string& str) { return std::stoi(str); });

        lines.push_back(history_sequence);
    });

    return lines;
}

int extrapolate_history_for_next_value(std::vector<int>& history_sequence)
{
    std::vector<std::vector<int>> extrapolations;
    extrapolations.push_back(history_sequence);
    bool is_all_zero = false;
    auto current_layer = history_sequence;
    while (!is_all_zero) {
        std::vector<int> new_layer;
        for (int i = 1; i < current_layer.size(); i++) {
            new_layer.push_back(current_layer[i] - current_layer[i - 1]);
        }
        // print_lines(&new_layer, true);
        extrapolations.push_back(new_layer);
        current_layer = new_layer;
        if (std::all_of(new_layer.begin(), new_layer.end(), [](auto& value) { return value == 0; })) {
            is_all_zero = true;
            break;
        }
    }

    int current_value = current_layer.back();
    for (int i = 1; i < extrapolations.size(); i++) {
        const int current_index = (int)extrapolations.size() - i - 1;
        current_value = (int)extrapolations.at(current_index).back() + current_value;
    }

    // print_line("");
    return current_value;
}

int extrapolate_history_for_previous_value(std::vector<int>& history_sequence)
{
    std::vector<std::vector<int>> extrapolations;
    extrapolations.push_back(history_sequence);
    bool is_all_zero = false;
    auto current_layer = history_sequence;
    while (!is_all_zero) {
        std::vector<int> new_layer;
        for (int i = 1; i < current_layer.size(); i++) {
            new_layer.push_back(current_layer[i] - current_layer[i - 1]);
        }
        // print_lines(&new_layer, true);
        extrapolations.push_back(new_layer);
        current_layer = new_layer;
        if (std::all_of(new_layer.begin(), new_layer.end(), [](auto& value) { return value == 0; })) {
            is_all_zero = true;
            break;
        }
    }

    int current_value = current_layer.front();
    for (int i = 1; i < extrapolations.size(); i++) {
        const int current_index = (int)extrapolations.size() - i - 1;
        current_value = (int)extrapolations.at(current_index).front() - current_value;
    }

    // print_line("");
    return current_value;
}

int extrapolate_history_sequences_for_total(std::vector<std::vector<int>>& history_sequences, bool is_backwards = false)
{
    int total = 0;
    std::for_each(history_sequences.begin(), history_sequences.end(), [&total, &is_backwards](auto& history) {
        total += !is_backwards ? extrapolate_history_for_next_value(history)
                               : extrapolate_history_for_previous_value(history);
    });
    return total;
}

int Nine::get_part_1_result(problem_lines ptr_lines)
{
    auto history_sequences = parse_input(ptr_lines);
    return extrapolate_history_sequences_for_total(history_sequences);
}

int Nine::get_part_2_result(problem_lines ptr_lines)
{
    auto history_sequences = parse_input(ptr_lines);
    return extrapolate_history_sequences_for_total(history_sequences, true);
}