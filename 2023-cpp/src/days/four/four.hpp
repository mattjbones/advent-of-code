#include "../../logging.tcc"
#include "../../runner.hpp"

using namespace std;

class Four : public RunnerBase {
public:
    Four(int part_1, int part_2);

protected:
    int get_part_1_result(problem_lines ptr_lines) override;
    int get_part_2_result(problem_lines ptr_lines) override;
};

struct Card {
    std::vector<int> winning_numbers;
    std::vector<int> player_numbers;
};

std::vector<int> calculate_copy_count(vector<int>& matching_per_card);
