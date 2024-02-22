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

struct CBT {
    size_t level;
    vector<vector<TreeNode*>> links;
};

class CBTInserter {
private:
    CBT cbt;

public:
    CBTInserter(TreeNode* root) {
        int depth = -1;
        queue<TreeNode*> q;
        q.push(root);
        while (!q.empty()) {
            depth++;
            cbt.links.push_back(vector<TreeNode*>());
            vector<TreeNode*> &link = cbt.links[depth];

            size_t len = q.size();
            for (size_t i = 0; i < len; i++) {
                TreeNode *curr = q.front();
                q.pop();

                link.push_back(curr);
                if (curr->left != nullptr) {
                    q.push(curr->left);
                }
                if (curr->right != nullptr) {
                    q.push(curr->right);
                }
            }
        }

        cbt.level = depth;
    }
    
    int insert(int val) {
        size_t linkNum = cbt.links.size();
        size_t l = cbt.level;
        while (l < linkNum) {
            vector<TreeNode*> &link = cbt.links[l];
            if (link.size() < (1 << l)) {
                break;
            }
            l++;
        }

        cbt.level = l;
        if (cbt.level == linkNum) {
            cbt.links.push_back(vector<TreeNode*>());
        }

        vector<TreeNode*> &link = cbt.links[cbt.level];
        TreeNode *nval = new TreeNode(val);
        link.push_back(nval);

        size_t nvalPos = link.size() - 1;
        size_t rootPos = nvalPos / 2;
        vector<TreeNode*> &rootLink = cbt.links[cbt.level - 1];
        TreeNode *rootNode = rootLink[rootPos];
        if (rootNode->left == nullptr) {
            rootNode->left = nval;
        } else {
            rootNode->right = nval;
        }

        return rootNode->val;
    }
    
    TreeNode* get_root() {
        return cbt.links[0][0];
    }
};