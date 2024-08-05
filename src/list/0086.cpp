#include <iostream>
using namespace std;


struct ListNode {
    int val;
    ListNode *next;
    ListNode() : val(0), next(nullptr) {}
    ListNode(int x) : val(x), next(nullptr) {}
    ListNode(int x, ListNode *next) : val(x), next(next) {}
};

class Solution {
public:
    ListNode* partition(ListNode* head, int x) {
        ListNode l1;
        ListNode l2;
        ListNode *p1 = &l1;
        ListNode *p2 = &l2;

        ListNode dummy(0, head);
        ListNode *curr = dummy.next;
        while (curr != nullptr) {
            // 将curr结点移动到新链表中
            if (curr->val < x) {
                p1->next = curr;
                p1 = p1->next;
            } else {
                p2->next = curr;
                p2 = p2->next;
            }

            // 将curr结点与后序结点断链
            dummy.next = curr->next;
            curr->next = nullptr;
            curr = dummy.next;
        }

        // 将链表2合并到链表1的末尾
        p1->next = l2.next;
        return l1.next;
    }
};