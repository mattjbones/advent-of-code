#include "../../logging.hpp"
#include "../../runner.hpp"

using namespace std;

class One : public RunnerBase {
public:
    void part_1(problem_lines lines, int expected);
    void part_2(problem_lines lines, int expected);
    One(filesystem::path path, int part_1, int part_2);
};