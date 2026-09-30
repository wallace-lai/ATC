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
    void do_max_depth1(TreeNode *root, int depth, int &max_depth) {
        if (root == nullptr) {
            return;
        }
        if (root->left == nullptr && root->right == nullptr) {
            max_depth = max(max_depth, depth + 1);
        }

        do_max_depth1(root->left, depth + 1, max_depth);
        do_max_depth1(root->right, depth + 1, max_depth);
    }

    // 分解
    int do_max_depth2(TreeNode *root) {
        if (root == nullptr) {
            return 0;
        }
        if (root->left == nullptr && root->right == nullptr) {
            return 1;
        }

        int left_max_depth = do_max_depth2(root->left);
        int right_max_depth = do_max_depth2(root->right);
        return 1 + max(left_max_depth, right_max_depth);
    }

    // 层序遍历
    int do_max_depth3(TreeNode *root) {
        if (root == nullptr) {
            return 0;
        }

        int depth = 0;
        queue<TreeNode *> q;
        q.push(root);
        while (!q.empty()) {
            depth++;

            int size = q.size();
            for (int i = 0; i < size; i++) {
                TreeNode *curr = q.front();
                q.pop();

                if (curr->left) {
                    q.push(curr->left);
                }
                if (curr->right) {
                    q.push(curr->right);
                }
            }
        }

        return depth;
    }

    
public:
    int maxDepth(TreeNode* root) {
        return do_max_depth3(root);
    }
};