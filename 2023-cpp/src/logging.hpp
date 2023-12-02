#ifndef __LOGGING_H_INCLUDED__
#define __LOGGING_H_INCLUDED__

#include <iostream>
#include <vector>
#include <string>
#include <filesystem>

using namespace std;

void print_char(char value);
void print_error(string line);
void print_line(string line);
void print_line(vector<string> *line);
void print_line(vector<int> *line);
vector<string> read_file_path(filesystem::path path);

#endif // __LOGGING_H_INCLUDED