#include <stack>
#include <vector>
#include <iostream>
#include <algorithm>
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
    vector<int> result;

public:
    vector<int> preorderTraversal(TreeNode* root) {
        if (root == nullptr) {
            return result;
        }
    
        stack<TreeNode*> stk;
        stk.push(root);
        TreeNode *curr;
        while (!stk.empty()) {
            curr = stk.top();
            stk.pop();
            result.push_back(curr->val);
            if (curr->right != nullptr) {
                stk.push(curr->right);
            }
            if (curr->left != nullptr) {
                stk.push(curr->left);
            }
        }

        return result;
    }
};

int main()
{
    TreeNode n1(1);
    TreeNode n2(2);
    TreeNode n3(3);
    TreeNode n4(4);
    TreeNode n5(5);
    TreeNode n6(6);
    TreeNode n7(7);

    n1.left = &n2;
    n1.right = &n3;

    n2.left = &n4;
    n2.right = &n5;
    n3.left = &n6;
    n3.right = &n7;

    Solution s;
    vector<int> ret;
    ret = s.preorderTraversal(&n1);

    for (auto &x : ret) {
        cout << x << " ";
    }
    cout << endl;

    return 0;
}