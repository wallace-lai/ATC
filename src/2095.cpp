#include <vector>
#include <string>
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
    ListNode* deleteMiddle(ListNode* head) {
        ListNode dummy;
        dummy.next = head;

        ListNode *slow = &dummy;
        ListNode *fast = &dummy;
        while (fast->next != nullptr && fast->next->next != nullptr) {
            fast = fast->next->next;
            slow = slow->next;
        }

        ListNode *toDel = slow->next;
        slow->next = slow->next->next;
        delete(toDel);

        return dummy.next;
    }
};

ListNode node[10];

int main()
{
    node[0].val = 1;
    node[0].next = &node[1];

    node[1].val = 2;
    node[1].next = &node[2];

    node[2].val = 3;
    node[2].next = &node[3];

    node[3].val = 4;
    node[3].next = &node[4];

    node[4].val = 5;
    node[4].next = &node[5];

    node[5].val = 6;
    node[5].next = &node[6];

    node[6].val = 7;
    node[6].next = &node[7];

    node[7].val = 8;

    ListNode *head = &node[0];
    Solution s;
    s.deleteMiddle(head);

    return 0;
}