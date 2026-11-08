struct ListNode {
    int val;
    ListNode *next;
    ListNode(int x) : val(x), next(nullptr) {}
};

class Solution {
public:
    ListNode *detectCycle(ListNode *head) {
        ListNode *slow = head;
        ListNode *fast = head;

        while (fast != nullptr) {
            // 环不存在，则 fast 会走完整条链表
            if (fast->next == nullptr) {
                return nullptr;
            }

            // fast 走的距离始终是 slow 的两倍
            slow = slow->next;
            fast = fast->next->next;

            // 若存在环，则 fast 必定会追上 slow
            if (fast == slow) {
                ListNode *ptr = head;
                while (ptr != slow) {
                    ptr = ptr->next;
                    slow = slow->next;
                }
                return ptr;
            }
        }

        // 理论上这条语句不可达
        return nullptr;
    }
};