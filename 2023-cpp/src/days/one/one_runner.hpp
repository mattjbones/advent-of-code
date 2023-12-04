#include "../../logging.tcc"
#include "../../runner.hpp"
#include <string>

using namespace std;

class One : public RunnerBase {
public:
    One(filesystem::path path, int part_1, int part_2, const char* day);

protected:
    int get_part_1_result(problem_lines) override;
    int get_part_2_result(problem_lines) override;
};