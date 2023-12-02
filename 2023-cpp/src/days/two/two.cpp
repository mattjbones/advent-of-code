#include <algorithm>
#include <array>
#include <boost/algorithm/string.hpp>
#include <numeric>
#include <regex>
#include <string>
#include <vector>

#include "../../logging.hpp"

#include "two.hpp"

using namespace std;

struct Set {
    int green;
    int red;
    int blue;
};

struct Game {
    vector<Set> sets;
};

const int red_limit = 12;
const int green_limit = 13;
const int blue_limit = 14;

std::vector<Game> get_games(problem_lines ptr_lines)
{
    vector<Game> games;
    vector<string> lines = *ptr_lines;
    const std::regex set_regex(R"( (\d*) (blue)|(\d*) (red)|(\d*) (green))");

    std::for_each(lines.begin(), lines.end(), [set_regex, &games](string& line) mutable {
        auto sets_start = line.find(':');
        string sets_string = line.substr(sets_start + 1);
        std::vector<std::string> sets;
        boost::split(sets, sets_string, [](char& character) { return character == ';'; });

        std::vector<Set> set_structs;
        std::for_each(sets.begin(), sets.end(), [set_regex, &set_structs](string& set) mutable {
            // print_line("Set string: " + set);
            std::smatch matches;
            string red;
            string green;
            string blue;
            string search(set);
            while (regex_search(search, matches, set_regex)) {
                blue = blue.empty() ? matches[1].str() : blue;
                red = red.empty() ? matches[3].str() : red;
                green = green.empty() ? matches[5].str() : green;
                search = matches.suffix();
            }

            set_structs.push_back(Set {
                !green.empty() ? std::stoi(green) : 0,
                !red.empty() ? std::stoi(red) : 0,
                !blue.empty() ? std::stoi(blue) : 0,
            });
            // print_line("Red: " + red + " Green: " + green + " Blue: " + blue);
        });

        games.push_back(Game { set_structs });
    });
    return games;
}

int TwoImpl::part_1(problem_lines ptr_lines)
{
    auto games = get_games(ptr_lines);

    std::vector<int> legal_games;
    for (int i = 0; i < games.size(); i++) {
        bool valid_game = true;
        // print_line("Game [" + to_string(i + 1) + "]");
        for (int set = 0; set < games[i].sets.size(); set++) {
            const Set current_set = games[i].sets[set];
            // print_line("Red: " + to_string(current_set.red) + " Green: " + to_string(current_set.green)
            // + " Blue: " + to_string(current_set.blue));
            if (current_set.blue > blue_limit or current_set.green > green_limit or current_set.red > red_limit) {
                // print_line("Invalid set");
                valid_game = false;
                break;
            }
        }
        if (valid_game) {
            legal_games.push_back(i + 1);
        }
    }

    return std::accumulate(legal_games.begin(), legal_games.end(), 0);
}

int TwoImpl::part_2(problem_lines ptr_lines)
{
    auto games = get_games(ptr_lines);
    std::vector<int> game_power;
    for (int i = 0; i < games.size(); i++) {
        // print_line("Game [" + to_string(i + 1) + "]");
        int max_red = 0;
        int max_green = 0;
        int max_blue = 0;
        for (int set = 0; set < games[i].sets.size(); set++) {
            const Set current_set = games[i].sets[set];
            // print_line("Red: " + to_string(current_set.red) + " Green: " + to_string(current_set.green)
            // + " Blue: " + to_string(current_set.blue));
            max_blue = current_set.blue > max_blue ? current_set.blue : max_blue;
            max_red = current_set.red > max_red ? current_set.red : max_red;
            max_green = current_set.green > max_green ? current_set.green : max_green;
        }
        game_power.push_back(max_red * max_blue * max_green);
    }

    return std::accumulate(game_power.begin(), game_power.end(), 0);
}