#include <stack>
#include <queue>
#include <vector>
#include <string>
#include <iostream>
#include <algorithm>
using namespace std;

class Solution {
public:
    int numWays(int n, vector<vector<int>>& relation, int k) {
        vector<vector<int>> graph;
        graph.resize(n);
        for (auto &x : relation) {
            graph[x[0]].push_back(x[1]);
            graph[x[1]].push_back(x[0]);
        }

        queue<int> q;
        q.push(0);
        int depth = -1;
        int result = 0;

        while (!q.empty()) {
            depth++;
            if (depth > k) {
                return result;
            }

            size_t len = q.size();
            for (size_t i = 0; i < len; i++) {
                int curr = q.front();
                q.pop();

                if (curr == n - 1 && depth == k) {
                    result++;
                }
                vector<int> &child = graph[curr];
                for (auto &x : child) {
                    q.push(x);
                }
            }
        }

        return result;
    }
};