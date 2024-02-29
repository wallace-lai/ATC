#include <map>
#include <set>
#include <stack>
#include <queue>
#include <vector>
#include <string>
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

class Solution {
private:
    ListNode *merge(ListNode *l1, ListNode *l2) {
        ListNode dummy(0, nullptr);
        ListNode *p = &dummy;

        while (l1 != nullptr && l2 != nullptr) {
            if (l1->val < l2->val) {
                p->next = l1;
                l1 = l1->next;
            } else {
                p->next = l2;
                l2 = l2->next;
            }

            p = p->next;
        }
        p->next = (l1 != nullptr ? l1 : l2);

        return dummy.next;
    }

    ListNode *sort(ListNode *head, ListNode *tail) {
        if (head == nullptr) {
            return head;
        }
        if (head->next == tail) {
            head->next = nullptr;
            return head;
        }

        ListNode *slow = head;
        ListNode *fast = head;
        while (fast != tail) {
            slow = slow->next;
            fast = fast->next;
            if (fast != tail) {
                fast = fast->next;
            }
        }
        ListNode *mid = slow;

        return merge(sort(head, mid), sort(mid, tail));
    }

public:
    ListNode* sortList(ListNode* head) {
        return sort(head, nullptr);
    }
};