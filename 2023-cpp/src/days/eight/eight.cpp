#include <algorithm>
#include <boost/regex/v5/regex.hpp>
#include <boost/regex/v5/regex_fwd.hpp>
#include <boost/regex/v5/regex_match.hpp>
#include <numeric>
#include <string>
#include <tuple>
#include <vector>

#include "eight.hpp"

Eight::Eight(int part_1, int part_2)
    : RunnerBase(filesystem::path("src/days/eight/"), part_1, part_2, "Eight", true) {};

const boost::regex direction_regex(R"(([1-9A-Z]{3}) = \(([1-9A-Z]{3}), ([1-9A-Z]{3})\))");

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

template <typename Functor>
int walk_graph_for_count(
    std::vector<char>& directions, problem_graph& graph, const char* start_position, Functor is_end_position)
{
    int count = 0;
    bool has_ended = false;
    string current_location = start_position;
    while (!has_ended) {
        for (const auto direction : directions) {
            count++;
            auto position
                = direction == *"L" ? std::get<0>(graph.at(current_location)) : std::get<1>(graph.at(current_location));

            // print_line("Current Location", position);
            if (is_end_position(position)) {
                has_ended = true;
                break;
            }

            current_location = position;
        }
    }

    return count;
}

const string END = "ZZZ";
int Eight::get_part_1_result(problem_lines ptr_lines)
{
    auto directions = parse_input_for_directions(ptr_lines);
    auto graph = parse_input_for_graph(ptr_lines);
    return walk_graph_for_count(directions, graph, "AAA", [](string& position) { return position == END; });
}

std::vector<string> get_starting_positions(problem_graph& graph)
{
    // find entires that end in an A
    std::vector<string> starting_positions;
    std::for_each(graph.begin(), graph.end(), [&starting_positions](auto& entry) {
        if (entry.first.back() == 'A') {
            starting_positions.push_back((entry.first));
        }
    });
    return starting_positions;
}

long calculate_value_of_lcm_for_starting_positions(std::vector<char>& directions, problem_graph& graph)
{
    auto starting_positions = get_starting_positions(graph);

    std::vector<int> counts;
    std::for_each(
        starting_positions.begin(), starting_positions.end(), [&directions, &graph, &counts](string& position) {
            const int count = walk_graph_for_count(
                directions, graph, position.c_str(), [](string& possible_end) { return possible_end.back() == 'Z'; });
            counts.push_back(count);
            print_line(position, count);
        });

    long current = counts.front();
    std::for_each(std::next(counts.begin(), 1), counts.end(), [&current](auto& number) {
        print_line(current, number);
        current = std::lcm(current, number);
    });

    print_line("result", current);

    return current;
}

void walk_for_patterns(std::vector<char>& directions, problem_graph graph)
{

    auto starting_positions = get_starting_positions(graph);

    int count = 0;
    bool has_ended = false;
    for (int i = 0; i < starting_positions.size(); i++) {
        count = 0;
        has_ended = false;
        auto current_location = starting_positions[i];
        print_line("\nChecking: ", current_location);
        while (!has_ended) {
            for (const auto direction : directions) {
                count++;

                auto position = direction == 'L' ? std::get<0>(graph.at(current_location))
                                                 : std::get<1>(graph.at(current_location));
                if (position.back() == 'Z') {
                    print_line("  ended  " + position + ", pos: ", count);
                }

                current_location = position;
                if (count > 100000) {
                    has_ended = true;
                    break;
                }
            }
        }
    }
}

int Eight::get_part_2_result(problem_lines ptr_lines)
{

    auto directions = parse_input_for_directions(ptr_lines);
    auto graph = parse_input_for_graph(ptr_lines);
    // walk_for_patterns(directions, graph);
    return (int)calculate_value_of_lcm_for_starting_positions(directions, graph);
}