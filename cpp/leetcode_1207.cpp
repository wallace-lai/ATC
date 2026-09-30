#include <vector>
#include <iostream>
#include <unordered_map>
#include <unordered_set>
using namespace std;

class Solution {
public:
    bool uniqueOccurrences(vector<int>& arr) {
        unordered_set<int> us;
        unordered_map<int,int> um;

        for (auto &i : arr) {
            um[i]++;
        }

        int counter = 0;
        for (auto it = um.cbegin(); it != um.cend(); it++) {
            counter++;
            us.insert(it->second);
        }
        if (counter != us.size()) {
            return false;
        }

        return true;
    }
};