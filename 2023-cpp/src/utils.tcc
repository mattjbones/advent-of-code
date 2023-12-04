template <class C, typename T> bool contains(C&& c, T e) { return find(begin(c), end(c), e) != end(c); };
