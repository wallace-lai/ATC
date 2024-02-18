#include <queue>
#include <stack>
#include <vector>
#include <string>
#include <climits>
#include <iostream>
#include <algorithm>
#include <unordered_map>
#include <unordered_set>
using namespace std;

class Solution {
private:
    void ProcessChild(int nx, int ny, queue<pair<int, int>> &q, unordered_set<string> &us) {
        string status = to_string(nx) + "*" + to_string(ny);
        if (us.count(status) == 0) {
            us.insert(status);
            q.push(make_pair(nx, ny));
        }
    }

public:
    bool canMeasureWater(int jug1Capacity, int jug2Capacity, int targetCapacity) {
        if (jug1Capacity + jug2Capacity < targetCapacity) {
            return false;
        }

        queue<pair<int, int>> q;
        q.push(make_pair(0, 0));
        unordered_set<string> us;
        us.insert("0*0");

        while (!q.empty()) {
            size_t len = q.size();
            for (size_t i = 0; i < len; i++) {
                pair<int, int> curr = q.front();
                q.pop();

                int x = curr.first;
                int y = curr.second;
                string status;
                int nx, ny;
                if (x != 0) {
                    nx = 0;
                    ny = y;
                    if (ny == targetCapacity) {
                        return true;
                    }
                    ProcessChild(nx, ny, q, us);
                }
                if (y != 0) {
                    nx = x;
                    ny = 0;
                    if (nx == targetCapacity) {
                        return true;
                    }
                    ProcessChild(nx, ny, q, us);
                }
                if (x < jug1Capacity) {
                    nx = jug1Capacity;
                    ny = y;
                    if (nx + ny == targetCapacity) {
                        return true;
                    }
                    ProcessChild(nx, ny, q, us);
                }
                if (y < jug2Capacity) {
                    nx = x;
                    ny = jug2Capacity;
                    if (nx + ny == targetCapacity) {
                        return true;
                    }
                    ProcessChild(nx, ny, q, us);
                }
                if (x >= jug2Capacity - y) {
                    nx = x - (jug2Capacity - y);
                    ny = jug2Capacity;
                    if (nx + ny == targetCapacity) {
                        return true;
                    }
                    ProcessChild(nx, ny, q, us);
                }
                if (x < jug2Capacity - y) {
                    nx = 0;
                    ny = y + x;
                    if (ny == targetCapacity) {
                        return true;
                    }
                    ProcessChild(nx, ny, q, us);
                }
                if (y >= jug1Capacity - x) {
                    nx = jug1Capacity;
                    ny = y - (jug1Capacity - x);
                    if (nx + ny == targetCapacity) {
                        return true;
                    }
                    ProcessChild(nx, ny, q, us);
                }
                if (y < jug1Capacity - x) {
                    nx = x + y;
                    ny = 0;
                    if (nx == targetCapacity) {
                        return true;
                    }
                    ProcessChild(nx, ny, q, us);
                }
            }
        }

        return false;
    }
};

/*
(0, 0)  --> (t, 0)
        --> (0, t)
        --> (x, y) x + y == target

(x, y)  
        // 1. 清空一个水壶
        --> (0, y)  x != 0
        --> (x, 0)  y != 0

        // 2. 装满一个水壶
        --> (Cx, y) x < Cx
        --> (x, Cy) y < Cy

        // 3. 一个水壶向另一个水壶倒水
        --> (x - (Cy - y), Cy)  x >= Cy - y
        --> (0, y + x)          x < Cy - y
        --> (Cx, y - (Cx - x))  y >= Cx - x
        --> (x + y, 0)          y < Cx - x
*/

int main()
{
    int Cx = 999911;
    int Cy = 999913;
    int target = 1;

    Solution s;
    bool ret = s.canMeasureWater(Cx, Cy, target);
    cout << "Result : " << (ret ? "true" : "false") << endl;

    return 0;
}