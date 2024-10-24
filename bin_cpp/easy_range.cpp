#include <iostream>
#include <vector>
#include <cstdint>

typedef int64_t i64;

int main() {
    int n; std::cin >> n;
    std::vector<i64> nums(n + 1, 0);
    nums[0] = nums[1] = 1;

    for (i64 i = 2; i < n + 1; i++) {
        if (i % 2 == 0) {
            nums[i] = nums[i / 2] + nums[i / 2 - 1];
        } else {
            nums[i] = nums[i / 2] - nums[i / 2 - 1];
        }
    }

    std::cout << nums[n] << std::endl;

    return 0;
}
