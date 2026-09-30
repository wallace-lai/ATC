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
    TreeNode *Build(vector<int> &post, int postBeg, int postEnd, vector<int> &in, int inBeg, int inEnd) {
        TreeNode *result = nullptr;

        if (postEnd - postBeg >= 1) {
            int root = post[postEnd - 1];
            int rootPos = pos[root];
            int leftSize = rootPos - inBeg;

            result = new TreeNode(root);
            result->left = Build(post, postBeg, postBeg + leftSize, in, inBeg, inBeg + leftSize);
            result->right = Build(post, postBeg + leftSize, postEnd - 1, in, inBeg + leftSize + 1, inEnd);
        }

        return result;
    }

public:
    TreeNode* buildTree(vector<int>& inorder, vector<int>& postorder) {
        for (size_t i = 0; i < inorder.size(); i++) {
            pos[inorder[i]] = i;
        }

        return Build(postorder, 0, postorder.size(), inorder, 0, inorder.size());
    }
};