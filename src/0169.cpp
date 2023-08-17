#include <vector>
#include <string>
#include <iostream>
#include <algorithm>
#include <unordered_set>
#include <unordered_map>
using namespace std;

class Solution {
public:
    int majorityElement(vector<int>& nums) {
        int candidate;
        int count = 0;

        for (auto i : nums) {
            if (count == 0) {
                candidate = i;
            }

            if (i == candidate) {
                count++;
            } else {
                count--;
            }
        }

        return candidate;
    }
};