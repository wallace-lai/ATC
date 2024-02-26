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
private:
    void DFS(TreeNode *root, vector<int> &val) {
        if (root == nullptr) {
            return;
        }

        DFS(root->left, val);
        val.push_back(root->val);
        DFS(root->right, val);
    }

public:
    int rangeSumBST(TreeNode* root, int low, int high) {
        vector<int> val;
        DFS(root, val);

        int beg = 0;
        int end = val.size() - 1;
        while (beg < val.size() && val[beg] < low) {
            beg++;
        }
        while (end >= 0 && val[end] > high) {
            end--;
        }

        if (end > beg) {
            return 0;
        }

        return accumulate()
    }
};
