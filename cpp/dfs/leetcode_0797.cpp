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
    void DFS(vector<vector<int>> &graph, size_t pos, size_t target, vector<int> &path, vector<vector<int>> &result) {
        if (pos == target) {
            result.push_back(path);
            return;
        }

        vector<int> &child = graph[pos];
        for (size_t i = 0; i < child.size(); i++) {
            path.push_back(child[i]);
            DFS(graph, child[i], target, path, result);
            path.pop_back();
        }
    }

public:
    vector<vector<int>> allPathsSourceTarget(vector<vector<int>>& graph) {
        vector<int> path;
        vector<vector<int>> result;
        size_t n = graph.size();

        path.push_back(0);
        DFS(graph, 0, n - 1, path, result);
        return result;
    }
};