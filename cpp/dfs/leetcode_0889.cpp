#include <set>
#include <map>
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
    TreeNode(int x) : val(x), left(NULL), right(NULL) {}
};

class Solution {
private:
    TreeNode* BuildTree(vector<int> &pre, int preBeg, int preEnd,
        vector<int> &post, int postBeg, int postEnd) {
        TreeNode *result = nullptr;
        if (preEnd - preBeg >= 1) {
            int root = pre[preBeg];
            result = new TreeNode(root);

            set<int> preLeft;
            set<int> postLeft;
            for (int i = 0; i + postBeg < postEnd - 1; i++) {
                preLeft.insert(pre[i + preBeg + 1]);
                postLeft.insert(post[i + postBeg]);
                if (preLeft == postLeft) {
                    result->left = BuildTree(pre, preBeg + 1, preBeg + 1 + i + 1, post, postBeg, postBeg + i + 1);
                    result->right = BuildTree(pre, preBeg + 1 + i + 1, preEnd, post, postBeg + i + 1, postEnd - 1);
                    break;
                }
            }
        }

        return result;
    }

public:
    TreeNode* constructFromPrePost(vector<int>& preorder, vector<int>& postorder) {
        return BuildTree(preorder, 0, preorder.size(), postorder, 0, postorder.size());
    }
};