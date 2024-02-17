#include <stack>
#include <queue>
#include <vector>
#include <string>
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
public:
    bool isUnivalTree(TreeNode* root) {
        int val = root->val;
        queue<TreeNode*> q;
        q.push(root);
        while (!q.empty()) {
            size_t len = q.size();
            for (size_t i = 0; i < len; i++) {
                TreeNode *curr = q.front();
                q.pop();

                if (curr->val != val) {
                    return false;
                }
                if (curr->left != nullptr) {
                    q.push(curr->left);
                }
                if (curr->right != nullptr) {
                    q.push(curr->right);
                }
            }
        }

        return true;
    }
};