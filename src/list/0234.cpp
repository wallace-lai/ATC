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

struct ListNode {
    int val;
    ListNode *next;
    ListNode() : val(0), next(nullptr) {}
    ListNode(int x) : val(x), next(nullptr) {}
    ListNode(int x, ListNode *next) : val(x), next(next) {}
};

// class Solution {
// public:
//     bool isPalindrome(ListNode* head) {
//         vector<int> val;
//         for (ListNode *p = head; p != nullptr; p = p->next) {
//             val.push_back(p->val);
//         }

//         size_t beg = 0;
//         size_t end = val.size() - 1;
//         while (beg < end) {
//             if (val[beg] != val[end]) {
//                 return false;
//             }
            
//             beg++;
//             end--;
//         }

//         return true;
//     }
// };

// class Solution {
//     void PreOrderTraverse(ListNode *head) {
//         if (head == nullptr) {
//             return;
//         }

//         cout << head->val << endl;
//         PreOrderTraverse(head->next);
//     }

//     void PostOrderTraverse(ListNode *head) {
//         if (head == nullptr) {
//             return;
//         }

//         PostOrderTraverse(head->next);
//         cout << head->val << endl;
//     }

//     void doCheck(ListNode **left, ListNode *right, bool *result) {
//         if (*result == false) {
//             return;
//         }
//         if (right == nullptr) {
//             return;
//         }

//         doCheck(left, right->next, result);
//         if ((*left)->val != right->val) {
//             *result = false;
//             return;
//         }
//         *left = (*left)->next;
//     }

// public:
//     bool isPalindrome(ListNode* head) {
//         ListNode *left = head;
//         bool result = true;
//         doCheck(&left, head, &result);
//         return result;
//     }
// };

class Solution {
    ListNode *reverseList(ListNode *head) {
        if (head == nullptr || head->next == nullptr) {
            return head;
        }

        ListNode *newHead = reverseList(head->next);
        head->next->next = head;
        head->next = nullptr;
        return newHead;
    }

public:
    bool isPalindrome(ListNode* head) {
        // 寻找中点
        ListNode *slow = head;
        ListNode *fast = head;
        while (fast != nullptr && fast->next != nullptr) {
            slow = slow->next;
            fast = fast->next->next;
        }
        if (fast != nullptr) {
            // 结点个数为奇数
            slow = slow->next;
        }

        // 逆序中点右侧部分链表
        ListNode *left = head;
        ListNode *right = reverseList(slow);

        // 开始比较
        while (right != nullptr) {
            if (left->val != right->val) {
                return false;
            }
            left = left->next;
            right = right->next;
        }

        return true;
    }
};