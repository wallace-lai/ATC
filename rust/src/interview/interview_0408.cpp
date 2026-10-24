struct TreeNode {
    int val;
    TreeNode *left;
    TreeNode *right;
    TreeNode(int x) : val(x), left(nullptr), right(nullptr) {}
};

class Solution {
public:
    TreeNode* lca(TreeNode* node, TreeNode* p, TreeNode* q) {
        if (!node) { return nullptr; }

        // 命中了p或者q，返回自身
        if (node->val == p->val || node->val == q->val) {
            return node;
        }

        TreeNode* left = lca(node->left, p, q);
        TreeNode* right = lca(node->right, p, q);

        // p和q分居两侧，当前即为LCA
        if (left != nullptr && right != nullptr) { return node; }

        // 只在一侧
        if (left != nullptr) { return left; }
        if (right != nullptr) { return right; }

        return nullptr;
    }

    TreeNode* lowestCommonAncestor(TreeNode* root, TreeNode* p, TreeNode* q) {
        return lca(root, p, q);
    }
};