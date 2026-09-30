#include <stack>
#include <queue>
#include <vector>
#include <string>
#include <iostream>
#include <algorithm>
using namespace std;

class Solution {
public:
    bool validPath(int n, vector<vector<int>>& edges, int source, int destination) {
        vector<vector<int>> graph;
        graph.resize(n);
        for (auto &x : edges) {
            graph[x[0]].push_back(x[1]);
            graph[x[1]].push_back(x[0]);
        }

        vector<bool> vis(n, false);
        queue<int> q;
        q.push(source);
        vis[source] = true;

        while (!q.empty()) {
            size_t len = q.size();
            for (size_t i = 0; i < len; i++) {
                int curr = q.front();
                q.pop();

                if (curr == destination) {
                    return true;
                }

                vector<int> &child = graph[curr];
                for (auto &x : child) {
                    if (!vis[x]) {
                        vis[x] = true;
                        q.push(x);
                    }
                }
            }
        }
        return false;
    }
};