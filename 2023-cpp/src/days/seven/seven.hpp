
#include <map>

#include "../../logging.tcc"
#include "../../runner.hpp"
#include "../../utils.tcc"

namespace Svn {

enum Card { Two, Three, Four, Five, Six, Seven, Eight, Nine, Ten, Jack, Queen, King, Ace };

enum Hand { HighCard, OnePair, TwoPair, ThreeOfAKind, FullHouse, FourOfAKind, FiveOfAKind };

const std::map<char, Card> CARD_MAP {
    { '2', Card::Two },
    { '3', Card::Three },
    { '4', Card::Four },
    { '5', Card::Five },
    { '6', Card::Six },
    { '7', Card::Seven },
    { '8', Card::Eight },
    { '9', Card::Nine },
    { 'T', Card::Ten },
    { 'J', Card::Jack },
    { 'Q', Card::Queen },
    { 'K', Card::King },
    { 'A', Card::Ace },
};

struct Game {
    std::map<Card, int> hand;
    std::vector<pair<Card, int>> sorted;
    int stake = 0;
    Hand hand_value;
    Card high_card = Card::Two;
    string hand_string;
};

};

class Seven : public RunnerBase {
public:
    Seven(int part_1 = -1, int part_2 = -1);

protected:
    int get_part_1_result(problem_lines ptr_lines) override;
    int get_part_2_result(problem_lines ptr_lines) override;

private:
    std::vector<Svn::Game> parse_input_for_games(problem_lines ptr_lines);
};
