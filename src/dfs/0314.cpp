#include <vector>
#include <string>
#include <climits>
#include <cmath>
#include <map>
#include <set>
#include <queue>
#include <stack>
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

bool cmp(const pair<int, vector<int>> &a, const pair<int, vector<int>> &b) {
    return a.first < b.first;
}

class Solution {
    unordered_map<int, vector<int>> um;

    void levelOrder(TreeNode *root) {
        if (root == nullptr) {
            return;
        }

        queue<pair<TreeNode *, int>> q;
        q.push(make_pair(root, 0));
        while (!q.empty()) {
            int size = q.size();
            for (int i = 0; i < size; i++) {
                pair<TreeNode *, int> curr = q.front();
                q.pop();

                if (um.find(curr.second) == um.end()) {
                    um[curr.second] = vector<int> {curr.first->val};
                } else {
                    um[curr.second].push_back(curr.first->val);
                }

                if (curr.first->left != nullptr) {
                    q.push(make_pair(curr.first->left, curr.second - 1));
                }
                if (curr.first->right != nullptr) {
                    q.push(make_pair(curr.first->right, curr.second + 1));
                }
            }
        }
    }

public:
    vector<vector<int>> verticalOrder(TreeNode* root) {
        levelOrder(root);
        vector<pair<int, vector<int>>> v(um.begin(), um.end());
        sort(v.begin(), v.end(), cmp);
        vector<vector<int>> result;
        for (auto &x : v) {
            result.push_back(x.second);
        }

        return result;
    }
};