#pragma once

#include <iterator>

template <class C, typename T> bool contains(C&& c, T e) { return find(begin(c), end(c), e) != end(c); };

template <class C, typename T> int index_of(C&& c, T e)
{
    auto index = distance(begin(c), find(begin(c), end(c), e));
    return index == std::size(c) ? -1 : index;
};
