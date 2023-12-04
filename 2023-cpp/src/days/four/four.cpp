#include <algorithm>
#include <array>
#include <boost/algorithm/string/classification.hpp>
#include <boost/algorithm/string/find_iterator.hpp>
#include <boost/algorithm/string/split.hpp>
#include <boost/regex/v5/regex.hpp>
#include <boost/regex/v5/regex_fwd.hpp>
#include <boost/regex/v5/regex_match.hpp>
#include <map>
#include <numeric>
#include <string>
#include <vector>

#include "../../logging.tcc"
#include "../../runner.hpp"
#include "../../utils.tcc"
#include "four.hpp"

using namespace std;

Four::Four(int part_1, int part_2)
    : RunnerBase(filesystem::path("src/days/four/"), part_1, part_2, "Four") {};

// left side is winning numbers | right side is your numbers
// Card 1: 41 48 83 86 17 | 83 86  6 31 17  9 48 53

const auto CARD_DELIM = ":";
const auto GAME_DELIM = "|";
// const boost::regex card_regex(R"((.*\S+):\s*.(?<player_numbers>\w.*) *| *(?<winning_numbers>\w.*))");
const boost::regex card_regex(R"(\w+.+\d*:\s+([\d +]+)\s+\|\s+([\d\ +]+))");

int calculate_score_from_match_count(auto target)
{
    const int first = 1;
    const int second = 2;
    std::vector<int> fibs = { first, second };
    while (fibs.size() < target) {
        fibs.push_back(fibs[fibs.size() - 1] * 2);
    }
    return target >= 1 ? fibs[target - 1] : 0;
}

vector<int> create_ints_from_score_line(auto line)
{
    vector<string> strings;
    vector<int> ints;
    boost::split(strings, line, boost::is_any_of(" "));

    std::for_each(strings.begin(), strings.end(), [&ints](auto number) {
        if (!number.empty() && number != " ") {
            ints.push_back(std::stoi(number));
        }
    });

    return ints;
}

Card create_card_from_line(auto player_numbers_string, auto winning_numbers_string)
{

    auto player_numbers = create_ints_from_score_line(player_numbers_string);
    auto winning_numbers = create_ints_from_score_line(winning_numbers_string);

    // print_line("\nPlayer: " + player_numbers_string);
    // print_lines(&player_numbers, true);

    // print_line("\nWinning: " + winning_numbers_string);
    // print_lines(&winning_numbers, true);

    return Card { winning_numbers, player_numbers };
}

std::vector<Card> get_cards(problem_lines ptr_lines)
{
    std::vector<Card> cards;
    std::for_each(ptr_lines->begin(), ptr_lines->end(), [&cards](auto& line) {
        boost::smatch card_parts;
        if (boost::regex_match(line, card_parts, card_regex)) {
            auto card = create_card_from_line(card_parts[1], card_parts[2]);
            cards.push_back(card);
        }
    });
    return cards;
}

std::vector<int> get_card_totals(std::vector<Card>& cards)
{
    std::vector<int> totals;
    std::for_each(cards.begin(), cards.end(), [&totals](Card& card) mutable {
        std::vector<int> matches;
        std::for_each(card.player_numbers.begin(), card.player_numbers.end(), [&matches, card](int number) mutable {
            if (contains(card.winning_numbers, number)) {
                matches.push_back(number);
            }
        });
        totals.push_back((int)matches.size());
        // print_line("Winning numbers: " + to_string(matches.size()));
    });
    return totals;
}

std::vector<int> get_card_scores(std::vector<int>& card_totals)
{
    std::vector<int> scores;
    std::for_each(card_totals.begin(), card_totals.end(), [&scores](int card_total) {
        if (card_total > 0) {
            auto score = calculate_score_from_match_count(card_total);
            // print_line("Score: " + to_string(score));
            scores.push_back(score);
        }
    });
    return scores;
}

int get_total(std::vector<int>& card_totals) { return std::accumulate(card_totals.begin(), card_totals.end(), 0); }

int Four::get_part_1_result(problem_lines ptr_lines)
{
    auto cards = get_cards(ptr_lines);
    auto totals = get_card_totals(cards);
    auto scores = get_card_scores(totals);

    return get_total(scores);
}

// original card 1 has 4 matches so wins copies of 2, 3, 4, 5
// original card 2 has 2 matches so wins copies of 3, 4
// copy of 2 also wins 3, 4

// Card 1: 41 48 83 86 17 | 83 86  6 31 17  9 48 53
// Card 2: 13 32 20 16 61 | 61 30 68 82 17 32 24 19

std::vector<int> calculate_copy_count(vector<int>& matching_per_card)
{
    std::vector<int> copy_counts(matching_per_card.size(), 1);

    for (auto i = 0; i < matching_per_card.size(); ++i) {
        int matches = matching_per_card[i];
        // print_line("Matches (Card " + to_string(i + 1) + "): " + to_string(matches));
        int current_card = i + 1;
        while (current_card <= i + matches) {
            // print_line("current card: " + to_string(current_card + 1));
            copy_counts.at(current_card) += copy_counts.at(i);
            ++current_card;
        }
        // print_lines(&copy_counts, true);
        // print_line("");
    }

    // print_line("Final copy Counts: ");
    // print_lines(&copy_counts, true);

    return copy_counts;
}

int Four::get_part_2_result(problem_lines ptr_lines)
{

    auto cards = get_cards(ptr_lines);
    auto totals = get_card_totals(cards);
    auto copy_counts = calculate_copy_count(totals);

    return get_total(copy_counts);
}