struct TreeNode {
    int val;
    TreeNode *left;
    TreeNode *right;
    TreeNode(int x) : val(x), left(nullptr), right(nullptr) {}
};


class Solution {
public:
    // 法一：中序遍历
    // 时间复杂度：O(n)，n 为节点个数
    // 按照中序访问节点，记录前驱，当前驱恰好是p时，当前节点就是答案
    // TreeNode* inorderSuccessor(TreeNode* root, TreeNode* p) {
    //     TreeNode* pre = nullptr;
    //     TreeNode* ans = nullptr;

    //     std::function<void(TreeNode*)> dfs = [&](TreeNode* node) {
    //         if (!node || ans) { return; }   // 节点为空或者已经找到答案，剪枝
            
    //         dfs(node->left);

    //         if (pre == p) { ans = node; }   // 前驱为 p ，当前即为后继
    //         pre = node;

    //         dfs(node->right);
    //     };

    //     dfs(root);
    //     return ans;
    // }

    // 法二：利用BST的性质迭代
    // 在BST中找到严格大于 p->val 的最小节点即是后继，用 ans 记录目前为止比 p->val 大的最小节点
    // 1. 若 cur->val > p->val，cur是一个候选后继，更新 ans = cur，然后往左走尝试更小的候选
    // 2. 若 cur->val <= p->val，cur不可能是后继，往右走
    TreeNode* inorderSuccessor(TreeNode* root, TreeNode* p) {
        TreeNode* ans = nullptr;
        TreeNode* cur = root;

        while (cur) {
            if (cur->val > p->val) {
                ans = cur;          // 候选，继续往左找更小的
                cur = cur->left;
            } else {
                cur = cur->right;   // 不够大，往右找
            }
        }

        return ans;
    }
};