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
    #if 0
    // 法一：自底向上递归（性能差）
    TreeNode *convert(TreeNode *root) {
        // 节点为空和叶子节点
        if (root == nullptr ||
            (root->left == nullptr && root->right == nullptr)) {
            return root;
        }

        TreeNode *l = convert(root->left);
        TreeNode *r = convert(root->right);

        if (l == nullptr) {
            root->left = nullptr;
            root->right = r;
            return root;
        } else {
            root->left = nullptr;
            root->right = r;

            TreeNode dummy = TreeNode { 0, nullptr, l };
            TreeNode *prev = &dummy;
            TreeNode *curr = prev->right;

            while (curr != nullptr) {
                prev = curr;
                curr = curr->right;
            }

            prev->right = root;
            return l;
        }
    }
    #endif

    // 法二：自底向上递归（性能改进）
    pair<TreeNode*, TreeNode*> convert(TreeNode *root) {
        if (root == nullptr) {
            return { nullptr, nullptr };
        }

        auto [lh, lt] = convert(root->left);
        auto [rh, rt] = convert(root->right);

        root->left = nullptr;
        root->right = rh;

        if (lt != nullptr) {
            lt->right = root;
        }

        TreeNode *head = lh ? lh : root;
        TreeNode *tail = rt ? rt : root;
        return { head, tail };
    }

    TreeNode* convertBiNode(TreeNode* root) {
        return convert(root).first;
    }
};