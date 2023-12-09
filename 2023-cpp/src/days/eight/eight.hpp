#include "../../logging.tcc"
#include "../../runner.hpp"
#include "../../utils.tcc"
#include <map>
#include <tuple>
#include <vector>

using namespace std;

using problem_graph = std::map<string, std::tuple<string, string>>;

class Eight : public RunnerBase {
public:
    Eight(int part_1 = -1, int part_2 = -1);

protected:
    int get_part_1_result(problem_lines ptr_lines) override;
    int get_part_2_result(problem_lines ptr_lines) override;

private:
    problem_graph parse_input_for_graph(problem_lines ptr_lines);
    std::vector<char> parse_input_for_directions(problem_lines ptr_lines);
};