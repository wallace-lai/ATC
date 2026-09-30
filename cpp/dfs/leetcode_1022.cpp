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
    TreeNode(int x) : val(x), left(NULL), right(NULL) {}
};

class Solution {
public:
    void DFS(TreeNode *root, int sum, int &result) {
        if (root->left == nullptr && root->right == nullptr) {
            int newSum = (sum << 1) + root->val;
            result += newSum;
            return;
        }

        int newSum = (sum << 1) + root->val;
        if (root->left != nullptr) {
            DFS(root->left, newSum, result);
        }
        if (root->right != nullptr) {
            DFS(root->right, newSum, result);
        }
    }

    int sumRootToLeaf(TreeNode* root) {
        int result = 0;
        int sum = 0;
        DFS(root, sum, result);
        return result;
    }
};