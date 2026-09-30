#include <queue>
#include <vector>
#include <iostream>
using namespace std;


struct ListNode {
    int val;
    ListNode *next;
    ListNode() : val(0), next(nullptr) {}
    ListNode(int x) : val(x), next(nullptr) {}
    ListNode(int x, ListNode *next) : val(x), next(next) {}
};

// class Solution {
// private:
//     ListNode *mergeTwoLists(ListNode *l1, ListNode *l2) {
//         ListNode dummy;
//         ListNode *p = &dummy;

//         while (l1 != nullptr && l2 != nullptr) {
//             if (l1->val < l2->val) {
//                 p->next = l1;
//                 l1 = l1->next;
//             } else {
//                 p->next = l2;
//                 l2 = l2->next;
//             }
//             p = p->next;
//         }

//         ListNode *remain = (l1 != nullptr ? l1 : l2);
//         while (remain != nullptr) {
//             p->next = remain;
//             remain = remain->next;
//             p = p->next;
//         }

//         return dummy.next;
//     }

//     ListNode *doMerge(vector<ListNode *> &lists, int left, int right) {
//         if (left == right) {
//             return lists[left];
//         }
//         if (left + 1 == right) {
//             return mergeTwoLists(lists[left], lists[right]);
//         }

//         int mid = left + (right - left) / 2;
//         return mergeTwoLists(doMerge(lists, left, mid), doMerge(lists, mid + 1, right));
//     }

// public:
//     ListNode* mergeKLists(vector<ListNode*>& lists) {
//         if (lists.size() == 0) {
//             return nullptr;
//         }

//         return doMerge(lists, 0, lists.size() - 1);
//     }
// };

class Solution {
    struct Compare {
        bool operator()(const ListNode *l, const ListNode *r) {
            return l->val > r->val;
        }
    };

public:
    ListNode* mergeKLists(vector<ListNode*>& lists) {
        if (lists.size() == 0) {
            return nullptr;
        }

        ListNode dummy;
        ListNode *curr = &dummy;

        priority_queue<ListNode *, vector<ListNode *>, Compare> pq;
        for (int i = 0; i < lists.size(); i++) {
            if (lists[i] != nullptr) {
                pq.push(lists[i]);
            }
        }

        while (!pq.empty()) {
            ListNode *minNode = pq.top();
            pq.pop();

            curr->next = minNode;
            if (minNode->next != nullptr) {
                pq.push(minNode->next);
            }
            curr = curr->next;
        }

        return dummy.next;
    }
};