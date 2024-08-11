#include <set>
#include <map>
#include <stack>
#include <queue>
#include <string>
#include <vector>
#include <sstream>
#include <iostream>
#include <algorithm>
#include <unordered_map>
#include <unordered_set>
using namespace std;

struct TreeNode {
    int val;
    TreeNode *left;
    TreeNode *right;
    TreeNode() : val(0), left(nullptr), right(nullptr) {}
    TreeNode(int x) : val(x), left(nullptr), right(nullptr) {}
    TreeNode(int x, TreeNode *left, TreeNode *right) : val(x), left(left), right(right) {}
};

class Solution {
private:
    // 分解
    int max_depth(TreeNode *root, int &result) {
        if (root == nullptr) {
            return 0;
        }

        int left_max_depth = max_depth(root->left, result);
        int right_max_depth = max_depth(root->right, result);
        result = max(result, left_max_depth + right_max_depth);
        return 1 + max(left_max_depth, right_max_depth);
    }
public:
    int diameterOfBinaryTree(TreeNode* root) {
        int result = 0;
        max_depth(root, result);
        return result;
    }
};