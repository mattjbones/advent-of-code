#include "../../logging.tcc"
#include "../../runner.hpp"

using namespace std;

class Three : public RunnerBase {
public:
    void part_1(problem_lines lines, int expected) override;
    void part_2(problem_lines lines, int expected) override;
    Three(filesystem::path path, int part_1, int part_2);
};