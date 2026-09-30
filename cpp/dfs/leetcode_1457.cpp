#include <set>
#include <map>
#include <stack>
#include <queue>
#include <string>
#include <vector>
#include <climits>
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
    TreeNode(int x) : val(x), left(NULL), right(NULL) {}
};

class Solution {
private:
    void DFS(TreeNode *root, vector<int> &p, int &result) {
        if (root->left == nullptr && root->right == nullptr) {
            p[0]++;
            p[root->val]++;

            if (IsValid(p)) {
                result++;
            }

            p[root->val]--;
            p[0]--;

            return;
        }
        
        if (root->left != nullptr) {
            p[0]++;
            p[root->val]++;
            DFS(root->left, p, result);
            p[root->val]--;
            p[0]--;
        }

        if (root->right != nullptr) {
            p[0]++;
            p[root->val]++;
            DFS(root->right, p, result);
            p[root->val]--;
            p[0]--;
        }
    }

    size_t GetOddNum(vector<int> &p) {
        size_t result = 0;

        for (size_t i = 1; i < p.size(); i++) {
            if (p[i] & 1 == 1) {
                result++;
            }
        }

        return result;
    }

    bool IsValid(vector<int> &p) {
        size_t oddNum = GetOddNum(p);
        if ((p[0] & 1) == 1 && oddNum != 1) {
            return false;
        }
        if ((p[0] & 1) == 0 && oddNum != 0) {
            return false;
        }
        return true;
    }
public:
    int pseudoPalindromicPaths (TreeNode* root) {
        int result = 0;
        vector<int> p;
        p.resize(10);
        DFS(root, p, result);
        return result;
    }
};