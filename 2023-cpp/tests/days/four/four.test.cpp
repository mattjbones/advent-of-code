#include <gtest/gtest.h>
#include <vector>

#include "../../../src/days/four/four.cpp"
#include "../../../src/runner.cpp"

namespace {

TEST(FourSuite, calculate_copy_count_one_winning_one_losing)
{

    std::vector<int> matching({ 1, 0 });
    const std::vector<int> expected = { 1, 2 };
    auto result = calculate_copy_count(matching);

    EXPECT_EQ(expected, result);
}

TEST(FourSuite, calculate_copy_count_two_winning_one_losing)
{
    std::vector<int> matching({ 1, 1, 0 });

    const std::vector<int> expected = { 1, 2, 3 };
    auto result = calculate_copy_count(matching);

    EXPECT_EQ(expected, result);
}

TEST(FourSuite, calculate_copy_count_two_winning_numbers_rest_losing)
{
    std::vector<int> matching({ 1, 2, 1, 0 });

    const std::vector<int> expected = {
        1,
        2,
        3,
        6,
    };
    auto result = calculate_copy_count(matching);

    EXPECT_EQ(expected, result);
}
}