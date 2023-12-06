
#include "six.hpp"
#include <algorithm>
#include <boost/algorithm/string/classification.hpp>
#include <boost/algorithm/string/join.hpp>
#include <boost/algorithm/string/split.hpp>
#include <climits>
#include <cstdlib>
#include <stdexcept>
#include <string>
#include <tuple>
#include <vector>

using namespace std;

Six::Six(int part_1, int part_2)
    : RunnerBase(filesystem::path("src/days/six/"), part_1, part_2, "Six") {};

struct Game {
    long time;
    long distance;
};

std::vector<Game> parse_input_for_games(problem_lines ptr_lines)
{
    std::vector<Game> games;
    std::for_each(ptr_lines->begin(), ptr_lines->end(), [&games](auto& line) mutable {
        std::vector<string> parts;
        boost::split(parts, line, boost::is_any_of(" "));
        int game_int = 0;
        std::for_each(std::next(parts.begin(), 1), parts.end(), [&games, &game_int](auto& part) mutable {
            if (!part.empty()) {
                try {
                    games.at(game_int).distance = std::stol(part);
                } catch (const std::out_of_range& _) {
                    games.push_back(Game { std::stol(part) });
                }
                ++game_int;
            }
        });
    });

    // std::for_each(games.begin(), games.end(), [](auto& game) {
    //     print_line("Game, time:" + to_string(game.time) + ", distance: " + to_string(game.distance));
    // });

    return games;
}

int get_max_number_of_winning_options(Game& game)
{
    std::vector<std::tuple<long, long>> winning_options;
    const long midpoint = game.time / 2;
    // print_line("Midpoint", midpoint);
    long option_time = LONG_MAX;
    long hold_time = midpoint;
    while (option_time > game.time && hold_time <= game.time) {
        const long travel_time = game.time - hold_time;
        option_time = travel_time * hold_time;
        if (option_time > game.distance) {
            winning_options.push_back({ option_time, hold_time });
        }
        ++hold_time;
    }

    option_time = INT_MAX;
    hold_time = midpoint - 1;
    while (option_time > game.time && hold_time >= 1) {
        const long travel_time = game.time - hold_time;
        option_time = travel_time * hold_time;
        if (option_time > game.distance) {
            winning_options.push_back({ option_time, hold_time });
        }
        --hold_time;
    }

    // std::for_each(winning_options.begin(), winning_options.end(), [](std::tuple<int, int>& option) {
    //     print_line("Winning: " + to_string(std::get<0>(option)) + ", " + to_string(std::get<1>(option)));
    // });

    return (int)winning_options.size();
}

std::vector<int> get_maxes_across_games(std::vector<Game>& games)
{
    std::vector<int> maxes;
    std::for_each(games.begin(), games.end(), [&maxes](Game& game) {
        auto score = get_max_number_of_winning_options(game);
        // print_line("Score: ", score);
        // print_line("");

        maxes.push_back(score);
    });

    return maxes;
}

int get_score_from_maxes(std::vector<int> maxes)
{

    int score = maxes.front();
    std::for_each(std::next(maxes.begin(), 1), maxes.end(), [&score](int& game_score) { score *= game_score; });
    return score;
}

int Six::get_part_1_result(problem_lines ptr_lines)
{
    auto games = parse_input_for_games(ptr_lines);
    auto maxes = get_maxes_across_games(games);
    return get_score_from_maxes(maxes);
};

Game parse_input_for_part_2_game(problem_lines ptr_lines)
{

    Game game {};
    int line_count = 0;
    std::for_each(ptr_lines->begin(), ptr_lines->end(), [&game, &line_count](auto& line) mutable {
        std::vector<string> parts;
        std::vector<string> number_parts;
        boost::split(parts, line, boost::is_any_of(" "));
        std::for_each(std::next(parts.begin(), 1), parts.end(), [&game, &line_count, &number_parts](auto& part) {
            if (part.empty()) {
                return;
            }
            number_parts.push_back(part);
        });

        const string number = boost::join(number_parts, "");

        if (line_count == 0) {
            game.time = std::stol(number);
        } else {
            game.distance = std::stol(number);
        }
        ++line_count;
    });

    print_line("Game, time:" + to_string(game.time) + ", distance: " + to_string(game.distance));
    return game;
}

int Six::get_part_2_result(problem_lines ptr_lines)
{
    auto game = parse_input_for_part_2_game(ptr_lines);
    auto score = get_max_number_of_winning_options(game);
    return score;
};