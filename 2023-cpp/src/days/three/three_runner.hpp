#include "../../logging.tcc"
#include "../../runner.hpp"
#include <boost/range/detail/common.hpp>

using namespace std;

class Three : public RunnerBase {
public:
    Three(filesystem::path path, int part_1, int part_2, const char* day);

protected:
    int get_part_1_result(problem_lines) override;
    int get_part_2_result(problem_lines) override;
};