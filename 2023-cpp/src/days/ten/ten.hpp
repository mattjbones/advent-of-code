

#include "../../logging.tcc"
#include "../../runner.hpp"
#include "../../utils.tcc"
#include <map>
#include <tuple>
#include <vector>

using namespace std;

class Ten : public RunnerBase {
public:
    Ten(int part_1 = -1, int part_2 = -1);

protected:
    int get_part_1_result(problem_lines ptr_lines) override;
    int get_part_2_result(problem_lines ptr_lines) override;

private:
};