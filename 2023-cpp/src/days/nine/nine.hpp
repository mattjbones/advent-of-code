
#include "../../logging.tcc"
#include "../../runner.hpp"
#include "../../utils.tcc"
#include <map>
#include <tuple>
#include <vector>

using namespace std;

class Nine : public RunnerBase {
public:
    Nine(int part_1 = -1, int part_2 = -1);

protected:
    int get_part_1_result(problem_lines ptr_lines) override;
    int get_part_2_result(problem_lines ptr_lines) override;

private:
    std::vector<std::vector<int>> parse_input(problem_lines ptr_lines);
};