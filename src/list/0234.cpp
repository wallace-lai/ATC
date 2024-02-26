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
public:
    bool isPalindrome(ListNode* head) {
        vector<int> val;
        for (ListNode *p = head; p != nullptr; p = p->next) {
            val.push_back(p->val);
        }

        size_t beg = 0;
        size_t end = val.size() - 1;
        while (beg < end) {
            if (val[beg] != val[end]) {
                return false;
            }
            
            beg++;
            end--;
        }

        return true;
    }
};