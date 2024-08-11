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
    // 遍历
    void do_preorder_traverse1(TreeNode *root, vector<int> &result) {
        if (root == nullptr) {
            return;
        }
        result.push_back(root->val);
        do_preorder_traverse1(root->left, result);
        do_preorder_traverse1(root->right, result);
    }

    // 分解
    vector<int> do_preorder_traverse2(TreeNode *root) {
        if (root == nullptr) {
            return {};
        }
        if (root->left == nullptr && root->right == nullptr) {
            return {root->val};
        }

        vector<int> left = do_preorder_traverse2(root->left);
        vector<int> right = do_preorder_traverse2(root->right);

        vector<int> result;
        result.push_back(root->val);
        result.insert(result.cend(), left.begin(), left.end());
        result.insert(result.cend(), right.begin(), right.end());
        return result;
    }

public:
    vector<int> preorderTraversal(TreeNode* root) {
        return do_preorder_traverse2(root);
    }
};