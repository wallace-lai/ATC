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

class Solution {
    ListNode *findReverseKth(ListNode *head, int k) {
        ListNode *slow = head;
        ListNode *fast = head;
        for (int i = 0; i < k; i++) {
            fast = fast->next;
        }
        while (fast != nullptr) {
            slow = slow->next;
            fast = fast->next;
        }

        return slow;
    }

public:
    ListNode* removeNthFromEnd(ListNode* head, int n) {
        // 单链表要删除倒数第n个结点，则要找到倒数第n + 1个结点
        // 当n和链表元素个数相等时，第n + 1个结点会指向链表第1个结点的前一个结点
        // 所以需要加上一个哑结点来表示第0个虚拟结点
        ListNode dummy(0, head);
        ListNode *prev = findReverseKth(&dummy, n + 1);
        prev->next = prev->next->next;
        return dummy.next;
    }
};