#include <vector>

using namespace std;

class Solution {
public:
    // 法一：Z 字形查找
    bool searchMatrix(vector<vector<int>>& matrix, int target) {
        int m = matrix.size();
        if (m == 0) { return false; }
        int n = matrix[0].size();
        if (n == 0) { return false; }

        // 从右上角开始
        int x = 0;
        int y = n - 1;

        while (x < m && y >= 0) {
            if (matrix[x][y] == target) {
                return true;
            } else if (matrix[x][y] > target) {
                // (x, y) 的右侧全部大于 matrix[x][y]，排除 y 这列
                y--;
            } else {
                // (x, y) 的上方全部小于 matrix[x][y]，排除 x 这行
                x++;
            }
        }

        return false;
    }
};