#include <vector>
#include <string>
#include <unordered_set>
using namespace std;

class Solution {
public:
    int equalPairs(vector<vector<int>>& grid) {
        unordered_multiset<string> rows;
        unordered_multiset<string> cols;
        auto n = grid.size();

        for (auto i = 0; i < n; i++) {
            string row;
            for (auto j = 0; j < n; j++) {
                row += to_string(grid[i][j]) + "-";
            }
            rows.insert(row);
        }
        for (auto i = 0; i < n; i++) {
            string col;
            for (auto j = 0; j < n; j++) {
                col += to_string(grid[j][i]) + "-";
            }
            cols.insert(col);
        }
        
        int ret = 0;
        for (auto &i : rows) {
            if (cols.find(i) != cols.end()) {
                ret += cols.count(i);
            }
        }
        return ret;
    }
};