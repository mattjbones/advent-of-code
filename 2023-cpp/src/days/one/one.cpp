#include <algorithm>
#include <iostream>
#include <numeric>
#include <regex>

#include "one.hpp"

using namespace std;

int OneImpl::part_1(problem_lines ptr_lines)
{
    vector<int> answers;

    vector<string> lines = *ptr_lines;
    for (string line : lines) {
        // print_line(line);
        string number_parts;

        for (char character : line) {
            try {
                int number = std::stoi(&character);
                if (number_parts.length() == 0 || number_parts.length() == 1) {
                    number_parts += character;
                } else {
                    number_parts.at(1) = character;
                }
            } catch (std::invalid_argument _) {
                continue;
            }
        }

        if (number_parts.length() == 1) {
            number_parts += number_parts;
        }

        answers.push_back(std::stoi(number_parts));
    }

    return std::accumulate(answers.begin(), answers.end(), 0);
}

int OneImpl::part_2(problem_lines ptr_lines)
{
    const vector<string> numbers = { "one", "two", "three", "four", "five", "six", "seven", "eight", "nine" };
    std::regex numberOrWord("([1-9]|one|two|three|four|five|six|seven|eight|nine)", std::regex_constants::ECMAScript);
    vector<int> answers;
    for (string line : *ptr_lines) {
        // print_line(line);

        string number_parts;
        std::smatch matches;
        string search(line);
        while (std::regex_search(search, matches, numberOrWord)) {
            auto match = matches[1].str();
            auto number_string_index = std::find(numbers.begin(), numbers.end(), match);
            string prefix = "";
            if (number_string_index != numbers.end()) {
                prefix = match.substr(1, match.length() - 1);
                match = std::to_string(number_string_index - numbers.begin() + 1);
            }

            if (number_parts.size() == 0 || number_parts.size() == 1) {
                number_parts += match;
            } else {
                number_parts.at(1) = match.at(0);
            }
            search = prefix + "" + matches.suffix().str();
        }

        if (number_parts.length() == 1) {
            number_parts += number_parts;
        }
        // cout << "Line: ";
        // cout << line;
        // cout << " parts: ";
        // cout << number_parts << endl;
        answers.push_back(std::stoi(number_parts));
    }

    return std::accumulate(answers.begin(), answers.end(), 0);
}