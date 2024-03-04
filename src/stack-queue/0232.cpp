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

class MyQueue {
private:
    stack<int> main;
    stack<int> backup;

public:
    MyQueue() {

    }
    
    void push(int x) {
        main.push(x);
    }
    
    int pop() {
        if (backup.empty()) {
            while (!main.empty()) {
                backup.push(main.top());
                main.pop();
            }
        }

        int ret = backup.top();
        backup.pop();
        return ret;
    }
    
    int peek() {
        if (backup.empty()) {
            while (!main.empty()) {
                backup.push(main.top());
                main.pop();
            }
        }

        return backup.top();
    }
    
    bool empty() {
        return main.empty() && backup.empty();
    }
};