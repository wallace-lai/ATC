#include <map>
#include <set>
#include <stack>
#include <queue>
#include <vector>
#include <string>
#include <memory>
#include <iostream>
#include <algorithm>
#include <unordered_map>
#include <unordered_set>
using namespace std;

// class MyStack {
// private:
//     queue<int> main;
//     queue<int> backup;

// public:
//     MyStack() {

//     }
    
//     void push(int x) {
//         backup.push(x);
//         while (!main.empty()) {
//             backup.push(main.front());
//             main.pop();
//         }

//         swap(main, backup);
//     }
    
//     int pop() {
//         int ret = main.front();
//         main.pop();
//         return ret;
//     }
    
//     int top() {
//         return main.front();
//     }
    
//     bool empty() {
//         return main.empty();
//     }
// };

class MyStack {
private:
    queue<int> q;

public:
    MyStack() {

    }
    
    void push(int x) {
        size_t len = q.size();
        q.push(x);
        for (size_t i = 0; i < len; i++) {
            int curr = q.front();
            q.pop();
            q.push(curr);
        }
    }
    
    int pop() {
        int ret = q.front();
        q.pop();
        return ret;
    }
    
    int top() {
        return q.front();
    }
    
    bool empty() {
        return q.empty();
    }
};

int main()
{
    MyStack stk;
    stk.push(1);
    stk.push(2);
    stk.push(3);
    cout << stk.top() << endl;
    cout << stk.pop() << endl;
    cout << stk.empty() << endl;

    return 0;
}