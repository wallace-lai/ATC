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
    int findBottomLeftValue(TreeNode* root) {
        int result;

        queue<TreeNode*> q;
        q.push(root);
        while (!q.empty()) {
            size_t len = q.size();
            for (size_t i = 0; i < len; i++) {
                TreeNode *curr = q.front();
                q.pop();

                if (i == 0) {
                    result = curr->val;
                }
                if (curr->left != nullptr) {
                    q.push(curr->left);
                }
                if (curr->right != nullptr) {
                    q.push(curr->right);
                }
            }
        }

        return result;
    }
};