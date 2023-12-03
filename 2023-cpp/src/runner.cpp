#include <filesystem>
#include <fstream>
#include <iostream>
#include <string>
#include <vector>

using namespace std;

vector<string> read_file_path(const filesystem::path& path)
{
    ifstream input_stream(path.c_str());

    if (!input_stream) {
        cerr << "Can't open input file!" << endl;
        cerr << path.c_str() << endl;
    }

    vector<string> text;
    string line;
    while (getline(input_stream, line)) {
        text.push_back(line);
    }
    return text;
}