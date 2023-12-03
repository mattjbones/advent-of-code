#ifndef __LOGGING_H_INCLUDED__
#define __LOGGING_H_INCLUDED__

#include <filesystem>
#include <iostream>
#include <string>
#include <vector>

using namespace std;

template <typename T> void print_error(T line) { cerr << line << endl; }
template <typename T> void print_line(T line) { cout << line << endl; }
template <typename T> void print_lines(vector<T>* lines)
{
    std::for_each(lines->begin(), lines->end(), [](T line) { print_line(line); });
}

#endif // __LOGGING_H_INCLUDED