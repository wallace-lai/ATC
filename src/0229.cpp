#include <vector>
#include <string>
#include <algorithm>
#include <iostream>
#include <unordered_set>
#include <unordered_map>
using namespace std;

// class Solution {
// public:
//     vector<int> majorityElement(vector<int>& nums) {
//         int n = nums.size();

//         unordered_set<int> ret;
//         unordered_map<int, int> um;
//         for (auto i : nums) {
//             um[i]++;
//             if (um[i] > n / 3) {
//                 ret.insert(i);
//             }
//         }

//         return vector<int>(ret.cbegin(), ret.cend());
//     }
// };

class Solution {
public:
    vector<int> majorityElement(vector<int>& nums) {
        int vote1 = 0;
        int vote2 = 0;
        int candidate1, candidate2;

        for (auto i : nums) {
            if (vote1 > 0 && i == candidate1) {
                vote1++;
            } else if (vote2 > 0 && i == candidate2) {
                vote2++;
            } else if (vote1 == 0) {
                candidate1 = i;
                vote1++;
            } else if (vote2 == 0) {
                candidate2 = i;
                vote2++;
            } else {
                vote1--;
                vote2--;
            }
        }

        int count1 = 0;
        int count2 = 0;
        for (auto i : nums) {
            if (vote1 > 0 && i == candidate1) {
                count1++;
            }
            if (vote2 > 0 && i == candidate2) {
                count2++;
            }
        }

        vector<int> ret;
        if (vote1 > 0 && count1 > nums.size() / 3) {
            ret.push_back(candidate1);
        }
        if (vote2 > 0 && count2 > nums.size() / 3) {
            ret.push_back(candidate2);
        }

        return ret;
    }
};

// class Solution {
// public:
//     vector<int> majorityElement(vector<int>& nums) {
//         vector<int> ans;
//         int element1 = 0;
//         int element2 = 0;
//         int vote1 = 0;
//         int vote2 = 0;

//         for (auto & num : nums) {
//             if (vote1 > 0 && num == element1) { //如果该元素为第一个元素，则计数加1
//                 vote1++;
//             } else if (vote2 > 0 && num == element2) { //如果该元素为第二个元素，则计数加1
//                 vote2++;
//             } else if (vote1 == 0) { // 选择第一个元素
//                 element1 = num;
//                 vote1++;
//             } else if (vote2 == 0) { // 选择第二个元素
//                 element2 = num;
//                 vote2++;
//             } else { //如果三个元素均不相同，则相互抵消1次
//                 vote1--;
//                 vote2--;
//             }
//         }

//         int cnt1 = 0;
//         int cnt2 = 0;
//         for (auto & num : nums) {
//             if (vote1 > 0 && num == element1) {
//                 cnt1++;
//             }
//             if (vote2 > 0 && num == element2) {
//                 cnt2++;
//             }
//         }
//         // 检测元素出现的次数是否满足要求
//         if (vote1 > 0 && cnt1 > nums.size() / 3) {
//             ans.push_back(element1);
//         }
//         if (vote2 > 0 && cnt2 > nums.size() / 3) {
//             ans.push_back(element2);
//         }

//         return ans;
//     }
// };


int main()
{
    Solution s;
    vector<int> nums{2, 2};
    vector<int> ret = s.majorityElement(nums);
    for (auto i : ret) {
        cout << i << " ";
    }
    cout << endl;

    return 0;
}