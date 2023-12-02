#include <iostream>
#include <vector>
#include <string>
#include <filesystem>
#include <fstream>
#include <algorithm>
#include <numeric>
#include <format>
#include <regex>

using namespace std;

using problem_lines = vector<string> *;

void print_char(char value);
void print_error(string line);
void print_line(string line);
void print_line(vector<string> *line);
void print_line(vector<int> *line);
vector<string> read_file_path(filesystem::path path);

class IRunner
{
public:
	void part_1_sample(problem_lines lines);
	void part_1_input(problem_lines lines);

	void part_2_sample(problem_lines lines, auto expected);
	void part_2_input(problem_lines lines);

	// TODO
	void part_1(problem_lines lines, auto expected);
	void part_2(problem_lines lines, auto expected);

	filesystem::path path;
};

class OneImpl
{
public:
	int part_1(problem_lines ptr_lines)
	{
		vector<int> answers;

		vector<string> lines = *ptr_lines;
		for (string line : lines)
		{
			// print_line(line);
			string number_parts;

			for (char character : line)
			{
				try
				{
					int number = std::stoi(&character);
					if (number_parts.length() == 0 || number_parts.length() == 1)
					{
						number_parts += character;
					}
					else
					{
						number_parts.at(1) = character;
					}
				}
				catch (std::invalid_argument _)
				{
					continue;
				}
			}

			if (number_parts.length() == 1)
			{
				number_parts += number_parts;
			}
			answers.push_back(std::stoi(number_parts));
		}

		return std::accumulate(answers.begin(), answers.end(), 0);
	}

	int part_2(problem_lines ptr_lines)
	{
		const vector<string> numbers = {"one", "two", "three", "four", "five", "six", "seven", "eight", "nine"};
		std::regex numberOrWord("([1-9]|one|two|three|four|five|six|seven|eight|nine)", std::regex_constants::ECMAScript);
		vector<int> answers;
		for (string line : *ptr_lines)
		{
			// print_line(line);

			string number_parts;
			std::smatch matches;
			string search(line);
			while (std::regex_search(search, matches, numberOrWord))
			{
				auto match = matches[1].str();
				auto number_string_index = std::find(numbers.begin(), numbers.end(), match);
				string prefix = "";
				if (number_string_index != numbers.end())
				{
					prefix = match.substr(1, match.length() - 1);
					match = std::to_string(number_string_index - numbers.begin() + 1);
				}

				if (number_parts.size() == 0 || number_parts.size() == 1)
				{
					number_parts += match;
				}
				else
				{
					number_parts.at(1) = match.at(0);
				}
				search = prefix + "" + matches.suffix().str();
			}

			if (number_parts.length() == 1)
			{
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
};

class One : public IRunner
{
public:
	One(filesystem::path targetPath)
	{
		path = targetPath;
	}

	void part_1_sample(problem_lines lines)
	{
		auto expected = 142;
		print_line("Part 1 sample");
		OneImpl one;
		auto result = one.part_1(lines);
		cout << "Result: ";
		cout << result << endl;
		if (result != expected)
		{
			print_error("Failed");
			exit(1);
		}
		cout << "Correct!" << endl;
		cout << endl;
	}

	void part_1_input(problem_lines lines)
	{
		auto expected = 54331;
		print_line("Part 1 input");
		OneImpl one;
		auto result = one.part_1(lines);
		cout << "Result: ";
		cout << result << endl;
		if (result != expected)
		{
			print_error("Failed");
			exit(1);
		}
		cout << "Correct!" << endl;
		cout << endl;
	}

	void part_2_sample(problem_lines lines, auto expected)
	{
		print_line("Part 2 sample");
		OneImpl one;
		auto result = one.part_2(lines);
		cout << "Result: ";
		cout << result << endl;
		if (result != expected)
		{
			cout << "Expected: ";
			cout << expected << endl;
			print_error("Failed");
			exit(1);
		}
		cout << "Correct!" << endl;
		cout << endl;
	}

	void part_2_input(problem_lines lines)
	{
		print_line("Part 2 input");
		auto expected = 0;
		OneImpl one;
		auto result = one.part_2(lines);
		cout << "Result: ";
		cout << result << endl;
		if (result != expected)
		{
			print_error("Failed");
			exit(1);
		}
		cout << "Correct!" << endl;
		cout << endl;
	}
};

int main()
{
	cout << "Advent of Code 2023!" << endl;
	cout << endl;

	One day_one(filesystem::path("./src/days/one/"));

	filesystem::path day_one_path_1_sample = filesystem::path(day_one.path) += filesystem::path("part_1/sample");
	vector<string> sample_data = read_file_path(day_one_path_1_sample);
	day_one.part_1_sample(&sample_data);

	filesystem::path day_one_path_1_input = filesystem::path(day_one.path) += filesystem::path("part_1/input");
	vector<string> input_data = read_file_path(day_one_path_1_input);
	day_one.part_1_input(&input_data);

	filesystem::path day_one_path_2_sample = filesystem::path(day_one.path) += filesystem::path("part_2/sample");
	vector<string> sample_data_2 = read_file_path(day_one_path_2_sample);

	vector<string> dummy_vec({"honemkmbfbnlhtbq19twonekbp", "twone", "threeight", "four444ninine", "fiveight", "six", "sevenine", "eightwo", "nineightwone"});
	// vector<string> dummy_vec({"four444ninine"});
	day_one.part_2_sample(&dummy_vec, 495);

	day_one.part_2_sample(&sample_data_2, 281);
	day_one.part_2_input(&input_data);
}

void print_line(string line)
{
	cout << line << endl;
}

void print_char(char value)
{
	cout << value << endl;
}

void print_line(vector<string> *ptr_lines)
{
	vector<string> lines = *ptr_lines;
	std::for_each(lines.begin(), lines.end(), [](string line)
								{ print_line(line); });
}

void print_line(vector<int> *ptr_lines)
{
	vector<int> lines = *ptr_lines;
	std::for_each(lines.begin(), lines.end(), [](int line)
								{ print_line(to_string(line)); });
}

void print_error(string line)
{
	cerr << line << endl;
}

vector<string> read_file_path(filesystem::path path)
{
	ifstream input_stream(path.c_str());

	if (!input_stream)
	{
		cerr << "Can't open input file!" << endl;
		cerr << path.c_str() << endl;
	}

	vector<string> text;
	string line;
	while (getline(input_stream, line))
	{
		text.push_back(line);
	}
	return text;
}