#include <map>
#include <set>
#include <stack>
#include <queue>
#include <vector>
#include <string>
#include <iostream>
#include <algorithm>
#include <unordered_map>
#include <unordered_set>
using namespace std;

class Solution {
private:
    bool IsValid2(int a, int b) {
        return (a == b);
    }

    bool IsValid3(int a, int b, int c) {
        return (a == b && b == c) || (a + 1 == b && b + 1 == c);
    }

public:
    bool validPartition(vector<int>& nums) {
        size_t len = nums.size();
        vector<int> dp(len + 1, false);

        dp[0] = true;
        for (size_t i = 2; i <= len; i++) {
            dp[i] = dp[i - 2] && IsValid2(nums[i - 2], nums[i - 1]);
            if (i >= 3) {
                dp[i] = dp[i] || (dp[i - 3] &&
                    IsValid3(nums[i - 3], nums[i - 2], nums[i - 1]));
            }
        }

        return dp[len];
    }
};