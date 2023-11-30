#include <iostream>
#include <vector>
#include <string>
#include <filesystem>

using namespace std;

class IRunner
{
public:
    void part_1_sample(string data);
    void part_1_input(string data);

    void part_2_sample(string data);
    void part_2_input(string data);

    filesystem::path path;
};

class One : public IRunner
{
public:
    One(filesystem::path targetPath)
    {
        path = targetPath;
    }

    void part_1_sample(string *data)
    {
        cout << "Part 1 sample" << endl;
        cout << *data << endl;
    }

    void part_1_input(string *data)
    {
        cout << "Part 1 input" << endl;
        cout << *data << endl;
    }

    void part_2_sample(string *data)
    {
        cout << "Part 2 sample" << endl;
        cout << *data << endl;
    }

    void part_2_input(string *data)
    {
        cout << "Part 2 input" << endl;
        cout << *data << endl;
    }
};

struct RunData
{
    IRunner runner;
    filesystem::path dirPath;
};

int main()
{
    cout << "Advent of Code 2023!" << endl;
    // std::vector<IRunner> days = {};
    // days.push_back();
    // std::for_each(days.begin(), days.end(), [](IRunner &runner)
    //               {
    //     cout << runner.path.c_str() << endl;
    //     string testData("hello");
    //     runner.part_1_input(testData); });

    string testData("hello");
    One dayOne(filesystem::path("./src/days/one/"));
    dayOne.part_1_sample(&testData);
    dayOne.part_1_input(&testData);
}
