#include <vector>
#include <stack>
#include <iostream>
#include <algorithm>
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
    vector<int> result;

public:
    vector<int> postorderTraversal(TreeNode* root) {
        if (root == nullptr) {
            return result;
        }

        stack<TreeNode*> stk;
        stk.push(root);
        TreeNode *curr;
        while (!stk.empty()) {
            curr = stk.top();
            stk.pop();
            result.push_back(curr->val);
            if (curr->left != nullptr) {
                stk.push(curr->left);
            }
            if (curr->right != nullptr) {
                stk.push(curr->right);
            }
        }

        reverse(result.begin(), result.end());
        return result;
    }
};