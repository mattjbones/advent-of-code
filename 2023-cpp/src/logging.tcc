#ifndef __LOGGING_H_INCLUDED__
#define __LOGGING_H_INCLUDED__

#include <boost/container/flat_set.hpp>
#include <filesystem>
#include <iostream>
#include <string>
#include <vector>

using namespace std;

template <typename T> void print_error(T line) { cerr << line << endl; }
template <typename T> void print_line(T line) { cout << line << endl; }
template <typename T, typename K> void print_line(T line, K next_line)
{
    cout << line;
    cout << " ";
    cout << next_line << endl;
}
template <typename T> void print_lines(vector<T>* lines, bool samelines = false)
{
    std::for_each(lines->begin(), lines->end(), [&samelines](T line) {
        if (!samelines) {
            print_line(line);
        } else {
            cout << line;
            cout << " ";
        }
    });
    if (samelines) {
        cout << endl;
    }
}

template <typename T> void print_lines(boost::container::flat_set<T>* lines, bool samelines = false)
{
    std::for_each(lines->begin(), lines->end(), [&samelines](T line) {
        if (!samelines) {
            print_line(line);
        } else {
            cout << line;
            cout << " ";
        }
    });
    if (samelines) {
        cout << endl;
    }
}

#endif // __LOGGING_H_INCLUDED