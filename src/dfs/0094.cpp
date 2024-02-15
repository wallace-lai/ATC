#include <stack>
#include <vector>
#include <algorithm>
#include <iostream>
using namespace std;

struct TreeNode {
    int val;
    TreeNode *left;
    TreeNode *right;
    TreeNode() : val(0), left(nullptr), right(nullptr) {}
    TreeNode(int x) : val(x), left(nullptr), right(nullptr) {}
    TreeNode(int x, TreeNode *left, TreeNode *right) : val(x), left(left), right(right) {}
};

// class Solution {
// public:
//     vector<int> inorderTraversal(TreeNode* root) {
//         DFS(root, result);
//         return result;
//     }
// private:
//     vector<int> result;
//     void DFS(TreeNode *root, vector<int> &result) {
//         if (root == nullptr) {
//             return;
//         }

//         DFS(root->left, result);
//         result.push_back(root->val);
//         DFS(root->right, result);
//     }
// };

class Solution {
public:
    vector<int> inorderTraversal(TreeNode* root) {
        stack<TreeNode*> stk;
        TreeNode *curr = root;
        while (curr != nullptr || !stk.empty()) {
            while (curr != nullptr) {
                stk.push(curr);
                curr = curr->left;
            }

            curr = stk.top();
            stk.pop();
            result.push_back(curr->val);
            curr = curr->right;
        }

        return result;
    }

private:
    vector<int> result;
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
    ret = s.inorderTraversal(&n1);

    for (auto &x : ret) {
        cout << x << " ";
    }
    cout << endl;

    return 0;
}