#include <array>
#include <string>
#include <vector>
#include <iostream>
#include <algorithm>
using namespace std;

class Solution {
public:
    bool closeStrings(string word1, string word2) {
        array<int, 26> cnt1;
        array<int, 26> cnt2;
        cnt1.fill(0);
        cnt2.fill(0);

        for (auto &i : word1) {
            cnt1[i - 'a']++;
        }
        sort(cnt1.begin(), cnt1.end());

        for (auto &i : word2) {
            cnt2[i - 'a']++;
        }
        sort(cnt2.begin(), cnt2.end());

        for (int i = 0; i < 26; i++) {
            if (cnt1[i] != cnt2[i]) {
                return false;
            }
        }

        return true;
    }
};