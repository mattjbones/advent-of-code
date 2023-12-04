#include "../../logging.tcc"
#include "../../runner.hpp"

using namespace std;

class Three : public RunnerBase {
public:
    Three(filesystem::path path, int part_1, int part_2);
};