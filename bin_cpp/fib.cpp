#include <cstdint>
#include <iostream>
#include <map>

typedef uint64_t u64;
typedef std::map<u64, u64> cache_t;

u64 fib_impl(u64 fib_n, cache_t &cache) {
  if (cache.find(fib_n) != cache.end()) {
    return cache[fib_n];
  }

  const u64 result = fib_impl(fib_n - 1, cache) + fib_impl(fib_n - 2, cache);

  cache[fib_n] = result;
  return result;
}

u64 fib(u64 fib_n) {
  cache_t cache;
  cache[0] = 0;
  cache[1] = 0;
  cache[2] = 1;

  return fib_impl(fib_n, cache);
}

int main() {
  u64 i;
  std::cin >> i;
  std::cout << fib(++i) << std::endl;

  return 0;
}
