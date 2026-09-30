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
    TreeNode(int x) : val(x), left(NULL), right(NULL) {}
};

class Solution {
public:
    void InOrder(TreeNode *root, vector<int> &val) {
        if (root == nullptr) {
            return;
        }

        InOrder(root->left, val);
        val.push_back(root->val);
        InOrder(root->right, val);
    }

    vector<int> GetRange(vector<int> &val, int q) {
        int minVal = -1;
        int maxVal = -1;
        auto iter = lower_bound(val.begin(), val.end(), q);
        if (iter != val.end()) {
            maxVal = *iter;
            if (*iter == q) {
                minVal = *iter;
                return vector<int>{minVal, maxVal};
            }
        }
        if (iter != val.begin()) {
            minVal = *(--iter);
        }

        return vector<int>{minVal, maxVal};
    }

public:
    vector<vector<int>> closestNodes(TreeNode* root, vector<int>& queries) {
        vector<int> val;
        InOrder(root, val);

        vector<vector<int>> result;
        result.resize(queries.size());
        for (size_t i = 0; i < queries.size(); i++) {
            result[i] = GetRange(val, queries[i]);
        }

        return result;
    }
};

int main()
{
    vector<int> val = {
        // 1, 2, 4, 6, 9, 13, 14, 15,
        4, 9,
    };

    int q = 3;

    Solution s;
    vector<int> ret = s.GetRange(val, q);
    for (auto &x : ret) {
        cout << x << " ";
    }
    cout << endl;

    return 0;
}