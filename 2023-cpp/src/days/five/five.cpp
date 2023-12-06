#include <boost/algorithm/string/classification.hpp>
#include <boost/algorithm/string/split.hpp>

#include <algorithm>
#include <climits>
#include <cstdlib>
#include <iostream>
#include <iterator>
#include <string>
#include <vector>

#include "five.hpp"

Five::Five(int part_1, int part_2)
    : RunnerBase(filesystem::path("src/days/five/"), part_1, part_2, "Five") {};

// seeds: 79 14 55 13

// seed-to-soil map:
// 50 98 2
// 52 50 48

// map is ranges:
//  dest start
//  source start
//  length

// seed to soil
// e.g 50 98 2
// dest 50 with 2 len 50 / 51
// source 98 with 2 length 98 / 99
// seed 98 -> 50
// seed 99 -> 51

// 52 50 48
// 52 with len 48, 52, 52, 53 ..
// 50 with len 48, 50, 51, 52 ...
// unmapped are 0 -> 0, 1 -> 1 etc

// map through each to get lowest location
// seed to soil
// soil to fertiliser
// fertiliser to water
// water to light
// light to temp
// temp to humid
// humid to location

struct MapEntry {
    long dest_start;
    long source_start;
    long length;
};

struct MapCategory {
    string name;
    vector<MapEntry> entries;
};

struct Almanac {
    std::vector<long> seeds;
    std::vector<MapCategory> categories;
};

struct Range {
    string name;
    long start = 0;
    long end = 0;
};

struct MapRange {
    string name;
    std::vector<Range> source;
    std::vector<Range> dest;
};

void print_entry(MapEntry& entry)
{
    print_line("Dest: " + to_string(entry.dest_start));
    print_line("Source: " + to_string(entry.source_start));
    print_line("Length: " + to_string(entry.length));
}

void print_category(MapCategory& cat, int display_count = 0)
{
    print_line("Name: " + cat.name);
    int count = 0;
    auto entries_end = display_count == 0 ? cat.entries.end() : std::next(cat.entries.begin(), display_count);
    std::for_each(cat.entries.begin(), entries_end, [&count](MapEntry& entry) {
        print_line("Category (" + to_string(count) + ")");
        print_entry(entry);
        print_line("");
        ++count;
    });
}

// void print_range(MapRange& range)
// {
//     print_line("name: " + range.name);
//     print_line("source_start: " + to_string(range.source_start));
//     print_line("source_end: " + to_string(range.source_end));
//     print_line("dest_start: " + to_string(range.dest_start));
//     print_line("dest_end: " + to_string(range.dest_end));
// }

void print_ranges(std::vector<MapRange>& ranges)
{
    int count = 0;
    std::for_each(ranges.begin(), ranges.end(), [&count](MapRange& range) {
        print_line("(" + to_string(count) + ")");
        // print_range(range);
        print_line("");
        ++count;
    });

    print_line("");
}

Almanac parse_input_for_seeds(problem_lines ptr_lines)
{
    Almanac almanac;
    string* seed_line = ptr_lines->data();
    std::vector<string> seed_parts;
    boost::split(seed_parts, *seed_line, boost::is_any_of(" "));
    std::for_each(std::next(seed_parts.begin(), 1), seed_parts.end(), [&almanac](auto& part) {
        if (!part.empty()) {
            almanac.seeds.push_back(std::stol(part));
        }
    });
    return almanac;
}

std::vector<MapCategory> parse_input_for_map_categories(problem_lines ptr_lines)
{
    std::vector<MapCategory> mapCategories;
    std::vector<string> parts;
    std::for_each(std::next(ptr_lines->begin(), 1), std::next(ptr_lines->end(), 1),
        [&parts, &mapCategories](string& line) mutable {
            if (!line.empty()) {
                parts.push_back(line);
            } else if (!parts.empty()) {
                // first line is name
                MapCategory category;
                auto name = parts.front();
                std::vector<string> name_parts;
                boost::split(name_parts, name, boost::is_any_of(" "));
                category.name = name_parts.front();
                // print_line("Category Name: " + category.name);

                // following lines are numbers
                std::for_each(std::next(parts.begin(), 1), parts.end(), [&category](auto& line) mutable {
                    std::vector<string> number_parts;
                    boost::split(number_parts, line, boost::is_any_of(" "));
                    const MapEntry entry {
                        std::stol(number_parts[0]),
                        std::stol(number_parts[1]),
                        std::stol(number_parts[2]),
                    };
                    category.entries.push_back(entry);
                });

                // print_category(category);
                mapCategories.push_back(category);
                parts.clear();
            }
        });

    return mapCategories;
}

Almanac parse_input_for_almanac(problem_lines ptr_lines)
{
    Almanac almanac = parse_input_for_seeds(ptr_lines);
    almanac.categories = parse_input_for_map_categories(ptr_lines);
    return almanac;
}

std::vector<MapRange> generated_map_ranges_from_almanac(Almanac& almanac)
{
    std::vector<MapRange> ranges;
    std::for_each(almanac.categories.begin(), almanac.categories.end(), [&ranges](MapCategory& category) mutable {
        // print_category(category);
        std::vector<Range> source_ranges;
        std::vector<Range> dest_ranges;

        std::for_each(category.entries.begin(), category.entries.end(),
            [&dest_ranges, &source_ranges, &category](MapEntry& entry) mutable {
                // print_entry(entry);

                source_ranges.push_back(Range {
                    category.name,
                    entry.source_start,
                    entry.source_start + entry.length - 1,
                });

                dest_ranges.push_back(Range {
                    category.name,
                    entry.dest_start,
                    entry.dest_start + entry.length - 1,
                });
                // print_ranges(category_ranges);
            });

        ranges.push_back(MapRange { category.name, source_ranges, dest_ranges });
    });

    return ranges;
}

long calculate_seed_location(long& seed_location, std::vector<MapRange>& ranges)
{
    // print_line("Checking seed: " + to_string(seed_location));
    long source = seed_location;
    long destination = 0;
    cout << "Seed " + to_string(seed_location) + ", ";
    std::for_each(ranges.begin(), ranges.end(), [&ranges, &source, &destination](MapRange& mapRange) mutable {
        bool found_matching_range = false;
        for (int i = 0; i < mapRange.source.size(); i++) {
            auto range = mapRange.source[i];
            auto dest_range = mapRange.dest[i];
            if (range.start <= source && source <= range.end) {
                found_matching_range = true;

                auto diff = dest_range.start - range.start;

                destination = source + diff;
                // do some maths;
                break;
            }
        }
        if (!found_matching_range) {
            destination = source;
        }
        cout << mapRange.name + " ";
        cout << destination;
        cout << ", ";
        // print_line("Seed: " + to_string(source) + " maps to soil: " + to_string(destination));
        source = destination;
        // exit(1);
    });

    cout << endl;
    // print_line("Source: " + to_string(source) + " Destination: " + to_string(destination));
    return destination;
}

long calculate_lowest_seed_location_from_ranges(std::vector<long>& seeds, std::vector<MapRange>& ranges)
{
    long lowest = LONG_MAX;

    // print_line("=== Ranges ===");
    // std::for_each(ranges.begin(), ranges.end(), [](std::vector<MapRange>& ranges) {
    //     print_ranges(ranges);
    //     print_line("\n####\n");
    // });

    std::for_each(seeds.begin(), seeds.end(), [&lowest, &ranges](long& seed_location) mutable {
        const long location = calculate_seed_location(seed_location, ranges);
        if (location < lowest) {
            lowest = location;
        }
    });

    return lowest;
}

int Five::get_part_1_result(problem_lines ptr_lines)
{

    auto almanac = parse_input_for_almanac(ptr_lines);
    auto ranges = generated_map_ranges_from_almanac(almanac);
    auto location = calculate_lowest_seed_location_from_ranges(almanac.seeds, ranges);

    print_line("Location: " + to_string(location));
    return (int)location;
}

std::vector<Range> generate_seeds_from_almanac(std::vector<long>& seed_ranges)
{
    std::vector<Range> ranges;
    for (int i = 0; i < seed_ranges.size(); i += 2) {

        const long start = seed_ranges[i];
        const long length = seed_ranges[i + 1];

        const Range seed_range({ "Seed", start, start + length - 1 });

        ranges.push_back(seed_range);
    }

    return ranges;
}

long calculate_seed_location_from_seed_range(Range& seed_range, std::vector<MapRange>& ranges)
{
    // print_line("Seed: " + to_string(seed_range.start) + ", " + to_string(seed_range.end));
    // auto first = ranges.front();
    // for(Range &source_ranges : first.source){

    // }

    //     for (int i = 0; i < first.source.size(); i++) {
    //         auto range = first.source[i];
    //         auto dest_range = first.dest[i];
    //         if (range.start <= source && source <= range.end) {

    //             auto diff = dest_range.start - range.start;

    //             destination = source + diff;
    //             // do some maths;
    //             break;
    //         }
    //     }

    return 0;
}

long calculate_lowest_seed_location_from_ranges(std::vector<Range>& seeds, std::vector<MapRange>& ranges)
{
    auto res = calculate_seed_location_from_seed_range(seeds.front(), ranges);
    return 0;
};

int Five::get_part_2_result(problem_lines ptr_lines)
{

    auto almanac = parse_input_for_almanac(ptr_lines);
    auto ranges = generated_map_ranges_from_almanac(almanac);
    auto seeds = generate_seeds_from_almanac(almanac.seeds);
    auto location = calculate_lowest_seed_location_from_ranges(seeds, ranges);

    print_line("Location: " + to_string(location));
    return (int)location;
}