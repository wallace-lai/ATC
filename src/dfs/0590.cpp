#include <queue>
#include <stack>
#include <vector>
#include <string>
#include <climits>
#include <iostream>
#include <algorithm>
#include <unordered_map>
#include <unordered_set>
using namespace std;

class Node {
public:
    int val;
    vector<Node*> children;

    Node() {}

    Node(int _val) {
        val = _val;
    }

    Node(int _val, vector<Node*> _children) {
        val = _val;
        children = _children;
    }
};

class Solution {
private:
    void DFS(Node *root, vector<int> &result) {
        if (root == nullptr) {
            return;
        }

        for (auto &x : root->children) {
            DFS(x, result);
        }
        result.push_back(root->val);
    }

public:
    vector<int> postorder(Node* root) {
        vector<int> result;
        DFS(root, result);
        return result;
    }
};