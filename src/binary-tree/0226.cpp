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
    void do_invert_tree1(TreeNode *root) {
        if (root == nullptr) {
            return;
        }

        TreeNode *tmp = root->left;
        root->left = root->right;
        root->right = tmp;

        do_invert_tree1(root->left);
        do_invert_tree1(root->right);
    }

    // 分解
    TreeNode *do_invert_tree2(TreeNode *root) {
        if (root == nullptr) {
            return nullptr;
        }

        TreeNode *left = do_invert_tree2(root->left);
        TreeNode *right = do_invert_tree2(root->right);
        root->left = right;
        root->right = left;
        return root;
    }

public:
    TreeNode* invertTree(TreeNode* root) {
        do_invert_tree1(root);
        return root;
    }
};