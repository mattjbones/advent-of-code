
#ifndef __RUNNER_H_INCLUDED__
#define __RUNNER_H_INCLUDED__

#include <filesystem>
#include <string>
#include <vector>

using namespace std;

using problem_lines = vector<string>*;

class IRunner {
public:
    virtual ~IRunner() = default;
    virtual void part_1(problem_lines lines, int expected) = 0;
    virtual void part_2(problem_lines lines, int expected) = 0;
};

class RunnerBase : public IRunner {
public:
    RunnerBase(filesystem::path data_path, int part_1, int part_2)
    {
        part_1_expected = part_1;
        part_2_expected = part_2;
        path = data_path;
    };

    filesystem::path path;
    int part_1_expected;
    int part_2_expected;
};

vector<string> read_file_path(const filesystem::path& path);

#endif //__RUNNER_H_INCLUDED