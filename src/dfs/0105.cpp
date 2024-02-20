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
private:
    unordered_map<int, int> pos;

private:
    TreeNode *BuildTree(vector<int> &pre, int preBeg, int preEnd, vector<int> &in, int inBeg, int inEnd) {
        TreeNode *root = nullptr;

        if (preEnd - preBeg >= 1) {
            root = new TreeNode(pre[preBeg]);
            int rootPos = pos[root->val];
            int leftSize = rootPos - inBeg;
            root->left = BuildTree(pre, preBeg + 1, preBeg + 1 + leftSize, in, inBeg, inBeg + leftSize);
            root->right = BuildTree(pre, preBeg + 1 + leftSize, preEnd, in, inBeg + leftSize + 1, inEnd);
        }

        return root;
    }

public:
    TreeNode* buildTree(vector<int>& preorder, vector<int>& inorder) {
        for (size_t i = 0; i < inorder.size(); i++) {
            pos[inorder[i]] = i;
        }

        return BuildTree(preorder, 0, preorder.size(), inorder, 0, inorder.size());
    }
};