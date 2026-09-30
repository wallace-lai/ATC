#include <queue>
#include <stack>
#include <vector>
#include <string>
#include <iostream>
#include <algorithm>
#include <unordered_map>
#include <unordered_set>
using namespace std;

class Solution {
private:
    bool IsSafe(size_t id, vector<vector<int>> &graph, unordered_set<int> &safeNode) {
        vector<int> &child = graph[id];
        for (auto &x : child) {
            if (safeNode.count(x) == 0) {
                return false;
            }
        }

        return true;
    }

public:
    vector<int> eventualSafeNodes(vector<vector<int>>& graph) {
        vector<vector<int>> rGraph;
        rGraph.resize(graph.size());
        for (size_t i = 0; i < graph.size(); i++) {
            vector<int> &g = graph[i];
            for (auto &x : g) {
                rGraph[x].push_back(i);
            }
        }

        queue<int> q;
        unordered_set<int> safeNode;
        for (size_t i = 0; i < graph.size(); i++) {
            vector<int> &g = graph[i];
            if (g.size() == 0) {
                safeNode.insert(i);
                q.push(i);
            }
        }

        while (!q.empty()) {
            size_t len = q.size();
            for (size_t i = 0; i < len; i++) {
                int curr = q.front();
                q.pop();

                vector<int> &child = rGraph[curr];
                for (auto &x : child) {
                    if (safeNode.count(x) == 0 && IsSafe(x, graph, safeNode)) {
                        safeNode.insert(x);
                        q.push(x);
                    }
                }
            }
        }

        vector<int> result(safeNode.begin(), safeNode.end());
        sort(result.begin(), result.end());
        return result;
    }
};