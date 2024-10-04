#include <vector>
#include <string>
#include <climits>
#include <cmath>
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
    double mtarget;
    void inorder(TreeNode *root, double &abs, double &result) {
        if (root == nullptr) {
            return;
        }

        inorder(root->left, abs, result);
        double dist = fabs(root->val - mtarget);
        if (dist < abs) {
            abs = dist;
            result = root->val;
        }
        inorder(root->right, abs, result);
    }
public:
    int closestValue(TreeNode* root, double target) {
        mtarget = target;
        double result = ULONG_MAX;
        double abs = ULONG_MAX;
        inorder(root, abs, result);
        return result;
    }
};