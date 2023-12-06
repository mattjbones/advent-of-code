
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

    virtual void run_input_part_1(const char* path) = 0;
    virtual void run_input_part_2(const char* path) = 0;

    virtual void run_input_part_1(const char* path, int expected) = 0;
    virtual void run_input_part_2(const char* path, int expected) = 0;

protected:
    virtual int get_part_1_result(problem_lines) = 0;
    virtual int get_part_2_result(problem_lines) = 0;

private:
    virtual vector<string> get_input_lines(filesystem::path path) = 0;
    virtual void runner_result_expected(int result, int expected) = 0;
};

class RunnerBase : public IRunner {
public:
    RunnerBase(filesystem::path data_path, int part_1, int part_2, const char* day, bool skippable = false)
        : path(std::move(data_path))
        , part_1_expected(part_1)
        , part_2_expected(part_2)
        , day_name(day)
        , skippable(skippable) {};

    void run_input_part_1(const char* target_data_path = "data/input") override;
    void run_input_part_2(const char* target_data_path = "data/input") override;

    void run_input_part_1(const char*, int) override;
    void run_input_part_2(const char*, int) override;

    string day_name;

protected:
    int get_part_1_result(problem_lines) override;
    int get_part_2_result(problem_lines) override;

    int get_part_1_expected();
    int get_part_2_expected();

private:
    int part_1_expected;
    int part_2_expected;
    filesystem::path path;
    bool skippable;

    vector<string> get_input_lines(filesystem::path path) override;
    void runner_result_expected(int result, int expected) override;
};

vector<string> read_file_path(const filesystem::path& path);

#endif //__RUNNER_H_INCLUDED