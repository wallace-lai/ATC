#include <set>
#include <map>
#include <stack>
#include <queue>
#include <string>
#include <vector>
#include <climits>
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
    TreeNode* expandBinaryTree(TreeNode* root) {
        queue<TreeNode*> q;
        q.push(root);

        while (!q.empty()) {
            size_t len = q.size();
            for (size_t i = 0; i < len; i++) {
                TreeNode *curr = q.front();
                q.pop();

                // non leaf node
                if (curr->left != nullptr || curr->right != nullptr) {
                    TreeNode *nLeft = new TreeNode(-1);
                    TreeNode *nRight = new TreeNode(-1);
                    nLeft->left = curr->left;
                    nRight->right = curr->right;

                    if (curr->left != nullptr) {
                        q.push(curr->left);
                        curr->left = nLeft;
                    }
                    if (curr->right != nullptr) {
                        q.push(curr->right);
                        curr->right = nRight;
                    }
                }
            }
        }

        return root;
    }
};