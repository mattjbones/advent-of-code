#include <algorithm>

#include "logging.hpp"

using namespace std;

void print_line(string line) { cout << line << endl; }

void print_char(char value) { cout << value << endl; }

void print_line(vector<string>* ptr_lines)
{
    vector<string> lines = *ptr_lines;
    std::for_each(lines.begin(), lines.end(), [](string line) { print_line(line); });
}

void print_line(vector<int>* ptr_lines)
{
    vector<int> lines = *ptr_lines;
    std::for_each(lines.begin(), lines.end(), [](int line) { print_line(to_string(line)); });
}

void print_error(string line) { cerr << line << endl; }
