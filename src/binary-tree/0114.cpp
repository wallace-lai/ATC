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
    // 分解1
    TreeNode *do_flatten1(TreeNode *root) {
        if (root == nullptr) {
            return nullptr;
        }

        TreeNode *left = do_flatten1(root->left);
        TreeNode *right = do_flatten1(root->right);
        root->left = nullptr;
        root->right = nullptr;

        TreeNode *prev = nullptr;
        TreeNode *curr = left;
        while (curr != nullptr) {
            prev = curr;
            curr = curr->right;
        }

        if (prev == nullptr) {
            root->right = right;
        } else {
            prev->right = right;
            root->right = left;
        }

        return root;
    }

    // 分解2
    TreeNode *do_flatten2(TreeNode *root) {
        if (root == nullptr) {
            return nullptr;
        }

        TreeNode *left = do_flatten2(root->left);
        TreeNode *right = do_flatten2(root->right);

        root->left = nullptr;
        root->right = left;

        TreeNode *curr = root;
        while (curr->right != nullptr) {
            curr = curr->right;
        }
        curr->right = right;

        return root;
    }

public:
    void flatten(TreeNode* root) {
        do_flatten2(root);
    }
};