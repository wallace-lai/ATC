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

class Solution {
public:
    int reachableNodes(int n, vector<vector<int>>& edges, vector<int>& restricted) {
        vector<bool> vis(n, false);
        unordered_set<int> us(restricted.begin(), restricted.end());

        unordered_map<int, vector<int>> graph;
        for (auto &x : edges) {
            graph[x[0]].push_back(x[1]);
            graph[x[1]].push_back(x[0]);
        }

        int result = 0;
        queue<int> q;
        q.push(0);
        while (!q.empty()) {
            int curr = q.front();
            q.pop();
            result++;
            vis[curr] = true;

            for (auto &x : graph[curr]) {
                if (!vis[x] && us.count(x) == 0) {
                    q.push(x);
                }
            }
        }

        return result;
    }
};