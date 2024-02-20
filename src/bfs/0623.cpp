#include <queue>
#include <stack>
#include <vector>
#include <string>
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
public:
    TreeNode* addOneRow(TreeNode* root, int val, int depth) {
        if (depth == 1) {
            TreeNode *result = new TreeNode(val);
            result->left = root;
            return result;
        }

        int count = 1;
        queue<TreeNode*> q;
        q.push(root);

        while (!q.empty()) {
            count++;

            size_t len = q.size();
            for (size_t i = 0; i < len; i++) {
                TreeNode *curr = q.front();
                q.pop();

                if (count == depth) {
                    TreeNode *nLeft = new TreeNode(val);
                    TreeNode *nRight = new TreeNode(val);
                    nLeft->left = curr->left;
                    nRight->right = curr->right;
                    curr->left = nLeft;
                    curr->right = nRight;
                    continue;
                }

                if (curr->left != nullptr) {
                    q.push(curr->left);
                }
                if (curr->right != nullptr) {
                    q.push(curr->right);
                }
            }

            if (count == depth) {
                break;
            }
        }

        return root;
    }
};