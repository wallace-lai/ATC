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
    ListNode *reverseList(ListNode *head) {
        ListNode dummy(0, nullptr);
        ListNode *tail;

        while (head != nullptr) {
            tail = head->next;
            head->next = dummy.next;
            dummy.next = head;
            head = tail;
        }

        return dummy.next;
    }

public:
    void reorderList(ListNode* head) {
        ListNode *slow = head;
        ListNode *fast = head;
        while (fast != nullptr && fast->next != nullptr) {
            slow = slow->next;
            fast = fast->next->next;
        }

        ListNode *l1 = head;
        ListNode *l2 = reverseList(slow->next);
        slow->next = nullptr;
        ListNode dummy(0, nullptr);
        ListNode *curr = &dummy;
        while (l1 != nullptr && l2 != nullptr) {
            curr->next = l1;
            l1 = l1->next;
            curr = curr->next;

            curr->next = l2;
            l2 = l2->next;
            curr = curr->next;
        }
        curr->next = l1;

        return;
    }
};