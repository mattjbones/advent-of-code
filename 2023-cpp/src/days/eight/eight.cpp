#include <algorithm>
#include <boost/regex/v5/regex.hpp>
#include <boost/regex/v5/regex_fwd.hpp>
#include <boost/regex/v5/regex_match.hpp>
#include <tuple>
#include <vector>

#include "eight.hpp"

Eight::Eight(int part_1, int part_2)
    : RunnerBase(filesystem::path("src/days/eight/"), part_1, part_2, "Eight") {};

const boost::regex direction_regex(R"(([A-Z]{3}) = \(([A-Z]{3}), ([A-Z]{3})\))");

std::vector<char> Eight::parse_input_for_directions(problem_lines ptr_lines)
{
    std::vector<char> directions;

    for (const char character : ptr_lines->front()) {
        directions.push_back(character);
    }

    return directions;
}

problem_graph Eight::parse_input_for_graph(problem_lines ptr_lines)
{
    problem_graph graph;

    std::for_each(std::next(ptr_lines->begin(), 2), ptr_lines->end(), [&graph](auto& line) {
        // print_line(line);
        boost::smatch direction_parts;
        if (boost::regex_match(line, direction_parts, direction_regex)) {
            // print_line(direction_parts[1]);
            // print_line(direction_parts[2], direction_parts[3]);
            graph.insert({ direction_parts[1], std::make_tuple(direction_parts[2], direction_parts[3]) });
        }
    });

    return graph;
}

const string END = "ZZZ";
int walk_graph_for_count(std::vector<char>& directions, problem_graph& graph)
{
    int count = 0;
    bool has_ended = false;
    string current_location = "AAA";
    while (!has_ended) {
        for (const auto direction : directions) {
            count++;
            auto position
                = direction == *"L" ? std::get<0>(graph.at(current_location)) : std::get<1>(graph.at(current_location));

            // print_line("Current Location", position);

            if (position == END) {
                has_ended = true;
                break;
            }

            current_location = position;
        }
    }

    return count;
}

int Eight::get_part_1_result(problem_lines ptr_lines)
{
    auto directions = parse_input_for_directions(ptr_lines);
    auto graph = parse_input_for_graph(ptr_lines);
    return walk_graph_for_count(directions, graph);
}

int Eight::get_part_2_result(problem_lines) { return 0; }