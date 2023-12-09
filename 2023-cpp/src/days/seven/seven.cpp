#include <__tuple_dir/tuple_element.h>
#include <algorithm>
#include <boost/algorithm/string/classification.hpp>
#include <boost/algorithm/string/split.hpp>
#include <cstdlib>
#include <exception>
#include <initializer_list>
#include <iterator>
#include <new>
#include <ranges>
#include <tuple>
#include <utility>
#include <vector>

#include "seven.hpp"

using namespace Svn;

Seven::Seven(int part_1, int part_2)
    : RunnerBase(filesystem::path("src/days/seven/"), part_1, part_2, "Seven") {};

string get_hand_name(Hand& hand)
{
    string hand_name;
    switch (hand) {
    case Hand::HighCard: {
        hand_name = "HighCard";
        break;
    }
    case Hand::OnePair: {
        hand_name = "OnePair";
        break;
    }
    case Hand::TwoPair: {
        hand_name = "TwoPair";
        break;
    }
    case Hand::ThreeOfAKind: {
        hand_name = "ThreeOfAKind";
        break;
    }
    case Hand::FullHouse: {
        hand_name = "FullHouse";
        break;
    }
    case Hand::FourOfAKind: {
        hand_name = "FourOfAKind";
        break;
    }
    case Hand::FiveOfAKind: {
        hand_name = "FiveOfAKind";
        break;
    }
    }
    return hand_name;
}

Hand parse_cards_for_hand(std::map<Card, int>& hand)
{
    Hand hand_value = Hand::HighCard;
    auto card = hand.begin();
    auto is_two = [](int value) { return value == 2; };

    switch (hand.size()) {
    case 1: {
        hand_value = Hand::FiveOfAKind;
        break;
    }

    case 2: {
        auto second_card = std::next(hand.begin(), 1);
        if (card->second == 4 || second_card->second == 4) {
            hand_value = Hand::FourOfAKind;
        } else if (card->second == 3 || second_card->second == 3) {
            hand_value = Hand::FullHouse;
        }
        break;
    }

    case 3: {
        auto second_card = std::next(hand.begin(), 1);
        auto third_card = std::next(hand.begin(), 2);
        auto counts = { card->second, second_card->second, third_card->second };

        std::vector<int> only_two;
        std::copy_if(counts.begin(), counts.end(), std::back_inserter(only_two), is_two);
        if (card->second == 3 || second_card->second == 3 || third_card->second == 3) {
            hand_value = Hand::ThreeOfAKind;
        } else if (only_two.size() == 2) {
            hand_value = Hand::TwoPair;
        }
    }

    case 4: {
        auto second_card = std::next(hand.begin(), 1);
        auto third_card = std::next(hand.begin(), 2);
        auto fourth = std::next(hand.begin(), 3);
        std::vector<int> counts = { card->second, second_card->second, third_card->second, fourth->second };

        std::vector<int> only_two;
        std::copy_if(counts.begin(), counts.end(), std::back_inserter(only_two), is_two);
        if (only_two.size() == 1) {
            hand_value = Hand::OnePair;
        }
        break;
    }
    }

    return hand_value;
}

std::vector<Game> Seven::parse_input_for_games(problem_lines ptr_lines)
{
    std::vector<Game> games;

    std::for_each(ptr_lines->begin(), ptr_lines->end(), [&games](auto& line) {
        Game game;
        std::vector<string> parts;
        boost::split(parts, line, boost::is_any_of(" "));

        for (const char character : parts[0]) {
            const Card card = CARD_MAP.at(character);
            if (game.hand.contains(card)) {
                game.hand.at(card)++;
            } else {
                game.hand.insert({ card, 1 });
            }
            if (game.high_card < card) {
                game.high_card = card;
            }
        }

        std::vector<pair<Card, int>> cards;
        cards.reserve(game.hand.size());
        for (auto& card : game.hand) {
            cards.emplace_back(card);
        }
        std::sort(cards.begin(), cards.end(), [](auto& one, auto& two) {
            if (one.first != two.first && one.second == two.second) {

                return one.first > two.first;
            }
            return one.second > two.second;
        });

        game.hand_value = parse_cards_for_hand(game.hand);
        game.stake = std::stoi(parts[1]);
        game.hand_string = parts[0];
        game.sorted = cards;
        games.push_back(game);

        // print_line("hand_value", game.hand_value);
        // print_line("hand", parts[0]);
        // print_line("");
    });

    return games;
}

bool compare_games(Game& one, Game& two)
{
    print_line("");
    print_line(one.hand_string, two.hand_string);
    print_line(one.hand_value, two.hand_value);

    if (one.hand_value != two.hand_value) {
        return one.hand_value < two.hand_value;
    }

    print_line("Evens");
    switch (one.hand_value) {
    case Hand::FiveOfAKind:
        return one.high_card < two.high_card;

    case Hand::FullHouse:
    case Hand::FourOfAKind: {
        auto one_one = std::next(one.sorted.begin(), 0);
        auto two_one = std::next(two.sorted.begin(), 0);
        if (one_one->first == two_one->first) {
            auto one_two = std::next(one.sorted.begin(), 1);
            auto two_two = std::next(two.sorted.begin(), 1);
            return one_two->first < two_two->first;
        } else {
            return one_one->first < two_one->first;
        }
        break;
    }
    case Hand::OnePair:
    case Hand::TwoPair:
    case Hand::ThreeOfAKind:
    case Hand::HighCard: {
        print_line("compare one");
        print_line(one.sorted.size());
        auto one_one = one.sorted.front();
        auto two_one = two.sorted.front();

        print_line("one_one", one_one.first);
        print_line("two_one", two_one.first);

        print_line(one_one.first, one_one.second);
        print_line(two_one.first, two_one.second);
        if (one_one.first != two_one.first) {
            return one_one.first < two_one.first;
        }

        print_line("compare two");
        auto one_two = std::next(one.sorted.begin(), 1);
        auto two_two = std::next(two.sorted.begin(), 1);
        if (one_two->first != two_two->first) {
            return one_two->first < two_two->first;
        }

        print_line("compare three");
        auto one_three = std::next(one.sorted.begin(), 2);
        auto two_three = std::next(two.sorted.begin(), 2);
        if (one_three->first != two_three->first) {
            return one_three->first < two_three->first;
        }

        print_line("compare four");
        auto one_four = std::next(one.sorted.begin(), 3);
        auto two_four = std::next(two.sorted.begin(), 3);
        if (one_four->first != two_four->first) {
            return one_four->first < two_four->first;
        }

        print_line("compare five");
        auto one_five = std::next(one.sorted.begin(), 4);
        auto two_five = std::next(two.sorted.begin(), 4);
        if (one_five->first != two_five->first) {
            return one_five->first < two_five->first;
        }

        return one.hand.size() == two.hand.size() && one.hand.size() == 5;
    }
    }
    return one.high_card < two.high_card;
}

void sort_games_by_rank(std::vector<Game>& games)
{
    // print_line("Sorting");
    std::sort(games.begin(), games.end(), compare_games);
    std::for_each(games.begin(), games.end(), [&games](Game& game) {
        print_line(get_hand_name(game.hand_value), game.hand_string);
        std::for_each(
            game.sorted.begin(), game.sorted.end(), [](auto& sorted) { print_line(sorted.first, sorted.second); });

        // if (game.hand_value == Hand::ThreeOfAKind && games.size() > 100) {
        //     exit(2);
        // }
    });
}

int score_games(std::vector<Game>& sorted_games)
{
    // print_line("Scoring");
    int score = 0;
    for (int i = 0; i < sorted_games.size(); i++) {
        // print_line(i, sorted_games[i].stake);
        score += sorted_games[i].stake * (i + 1);
    }
    return score;
}

int Seven::get_part_1_result(problem_lines ptr_lines)
{
    auto games = parse_input_for_games(ptr_lines);
    // print_line("Games:", games.size());
    sort_games_by_rank(games);
    return score_games(games);
};

int Seven::get_part_2_result(problem_lines) { return 0; };
