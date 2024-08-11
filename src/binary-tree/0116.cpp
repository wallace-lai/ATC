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
    TreeNode() : val(0), left(nullptr), right(nullptr) {}
    TreeNode(int x) : val(x), left(nullptr), right(nullptr) {}
    TreeNode(int x, TreeNode *left, TreeNode *right) : val(x), left(left), right(right) {}
};

struct Node {
public:
    int val;
    Node* left;
    Node* right;
    Node* next;

    Node() : val(0), left(NULL), right(NULL), next(NULL) {}

    Node(int _val) : val(_val), left(NULL), right(NULL), next(NULL) {}

    Node(int _val, Node* _left, Node* _right, Node* _next)
        : val(_val), left(_left), right(_right), next(_next) {}
};

class Solution {
private:
    // 遍历
    void do_connect1(Node *left, Node *right) {
        if (left == nullptr || right == nullptr) {
            return;
        }

        left->next = right;
        do_connect1(left->left, left->right);
        do_connect1(left->right, right->left);
        do_connect1(right->left, right->right);
    }

    // 层序遍历
    void do_connect2(Node *root) {
        if (root == nullptr) {
            return;
        }

        queue<Node *> q;
        q.push(root);
        while (!q.empty()) {
            int size = q.size();

            Node *prev = nullptr;
            Node *curr = nullptr;
            for (int i = 0; i < size; i++) {
                curr = q.front();
                q.pop();

                if (i == 0) {
                    prev = curr;
                } else {
                    prev->next = curr;
                    prev = curr;
                }

                if (curr->left) {
                    q.push(curr->left);
                }
                if (curr->right) {
                    q.push(curr->right);
                }
            }
        }
    }
public:
    Node* connect(Node* root) {
        do_connect2(root);
        return root;
    }
};