#include <cstdint>
#include <iostream>
#include <map>

typedef uint64_t u64;
typedef std::map<u64, u64> cache_t;

u64 trib_impl(u64 trib_n, cache_t &cache) {
  if (cache.find(trib_n) != cache.end()) {
    return cache[trib_n];
  }

  const u64 result = trib_impl(trib_n - 1, cache) +
                     trib_impl(trib_n - 2, cache) +
                     trib_impl(trib_n - 3, cache);

  cache[trib_n] = result;
  return result;
}

u64 trib(u64 trib_n) {
  cache_t cache;
  cache[0] = 0;
  cache[1] = 0;
  cache[2] = 1;

  return trib_impl(trib_n, cache);
}

int main() {
  u64 i;
  std::cin >> i;
  i += 2;
  std::cout << trib(i) << std::endl;

  return 0;
}
