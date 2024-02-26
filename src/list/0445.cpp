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

class Solution {
private:
    ListNode *ReverseList(ListNode *head) {
        if (head == nullptr || head->next == nullptr) {
            return head;
        }

        ListNode *sub = ReverseList(head->next);
        head->next->next = head;
        head->next = nullptr;
        return sub;
    }

public:
    ListNode* addTwoNumbers(ListNode* l1, ListNode* l2) {
        ListNode *la = ReverseList(l1);
        ListNode *lb = ReverseList(l2);

        int carry = 0;
        int sum = 0;
        ListNode dummy(0);
        while (la != nullptr && lb != nullptr) {
            sum = la->val + lb->val + carry;
            carry = sum / 10;
            sum = sum % 10;

            ListNode *newNode = new ListNode(sum);
            newNode->next = dummy.next;
            dummy.next = newNode;

            la = la->next;
            lb = lb->next;
        }

        ListNode *remain = (la != nullptr ? la : lb);
        while (remain != nullptr) {
            sum = remain->val + carry;
            carry = sum / 10;
            sum = sum % 10;

            ListNode *newNode = new ListNode(sum);
            newNode->next = dummy.next;
            dummy.next = newNode;

            remain = remain->next;
        }
        if (carry != 0) {
            ListNode *newNode = new ListNode(carry);
            newNode->next = dummy.next;
            dummy.next = newNode;
        }

        return dummy.next;
    }
};