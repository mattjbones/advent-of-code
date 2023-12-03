#include "../../logging.tcc"
#include "../../runner.hpp"

using namespace std;

class Two : public RunnerBase {
public:
    void part_1(problem_lines lines, int expected);
    void part_2(problem_lines lines, int expected);
    Two(filesystem::path path, int part_1, int part_2);
};