
#include "../../logging.tcc"
#include "../../runner.hpp"
#include "../../utils.tcc"

using namespace std;

class Six : public RunnerBase {
public:
    Six(int part_1 = -1, int part_2 = -1);

protected:
    int get_part_1_result(problem_lines ptr_lines) override;
    int get_part_2_result(problem_lines ptr_lines) override;
};