#include <queue>
#include <stack>
#include <vector>
#include <string>
#include <iostream>
#include <algorithm>
#include <unordered_map>
#include <unordered_set>
using namespace std;

// class Solution {
// private:
//     void FindIsland(vector<vector<int>> &grid, int x, int y, unordered_set<int> &island, vector<vector<bool>> &vis) {
//         int n = grid.size();
//         static int dx[] = {-1, 1, 0, 0};
//         static int dy[] = {0, 0, -1, 1};

//         int nx, ny;
//         for (size_t k = 0; k < 4; k++) {
//             nx = x + dx[k];
//             ny = y + dy[k];
//             if (0 <= nx && nx < n && 0 <= ny && ny < n && grid[nx][ny] == 1 && !vis[nx][ny]) {
//                 island.insert(nx * n + ny);
//                 vis[nx][ny] = true;
//                 FindIsland(grid, nx, ny, island, vis);
//             }
//         }
//     }

// public:
//     int shortestBridge(vector<vector<int>>& grid) {
//         int n = grid.size();
//         vector<vector<bool>> vis(n, vector<bool>(n, false));

//         unordered_set<int> src;
//         unordered_set<int> dst;
//         for (size_t i = 0; i < n; i++) {
//             for (size_t j = 0; j < n; j++) {
//                 if (grid[i][j] == 1 && !vis[i][j]) {
//                     src.insert(i * n + j);
//                     vis[i][j] = true;
//                     FindIsland(grid, i, j, src, vis);
//                     goto FIND_SRC_END;
//                 }
//             }
//         }
// FIND_SRC_END:

//         for (size_t i = 0; i < n; i++) {
//             for (size_t j = 0; j < n; j++) {
//                 if (grid[i][j] == 1 && !vis[i][j]) {
//                     dst.insert(i * n + j);
//                     vis[i][j] = true;
//                     FindIsland(grid, i, j, dst, vis);
//                     goto FIND_DST_END;
//                 }
//             }
//         }
// FIND_DST_END:

//         int count = -1;
//         queue<pair<int, int>> q;
//         for (auto iter = src.begin(); iter != src.end(); iter++) {
//             int x = (*iter) / n;
//             int y = (*iter) % n;
//             q.push(make_pair(x, y));
//         }

//         static int nx, ny;
//         static int dx[] = {-1, 1, 0, 0};
//         static int dy[] = {0, 0, -1, 1};

//         while (!q.empty()) {
//             count++;
//             size_t len = q.size();
//             for (size_t i = 0; i < len; i++) {
//                 pair<int, int> curr = q.front();
//                 q.pop();

//                 for (size_t k = 0; k < 4; k++) {
//                     nx = curr.first + dx[k];
//                     ny = curr.second + dy[k];
//                     if (0 <= nx && nx < n && 0 <= ny && ny < n) {
//                         if (dst.count(nx * n + ny) == 1) {
//                             return count;
//                         }

//                         if (!vis[nx][ny]) {
//                             vis[nx][ny] = true;
//                             q.push(make_pair(nx, ny));
//                         }
//                     }
//                 }
//             }
//         }

//         return count;
//     }
// };

class Solution {
private:
    void FindIsland(vector<vector<int>> &grid, int x, int y, vector<pair<int, int>> &island, vector<vector<bool>> &vis) {
        int n = grid.size();
        static int dx[] = {-1, 1, 0, 0};
        static int dy[] = {0, 0, -1, 1};

        int nx, ny;
        for (size_t k = 0; k < 4; k++) {
            nx = x + dx[k];
            ny = y + dy[k];
            if (0 <= nx && nx < n && 0 <= ny && ny < n && grid[nx][ny] == 1 && !vis[nx][ny]) {
                island.push_back(make_pair(nx, ny));
                vis[nx][ny] = true;
                grid[nx][ny] = 0;
                FindIsland(grid, nx, ny, island, vis);
            }
        }
    }

public:
    int shortestBridge(vector<vector<int>>& grid) {
        int n = grid.size();
        vector<vector<bool>> vis(n, vector<bool>(n, false));

        vector<pair<int, int>> source;
        for (size_t i = 0; i < n; i++) {
            for (size_t j = 0; j < n; j++) {
                if (grid[i][j] == 1) {
                    source.push_back(make_pair(i, j));
                    vis[i][j] = true;
                    grid[i][j] = 0;
                    FindIsland(grid, i, j, source, vis);
                    goto FIND_SOURCE_END;
                }
            }
        }

FIND_SOURCE_END:
        int count = -1;
        queue<pair<int, int>> q;
        for (auto iter = source.begin(); iter != source.end(); iter++) {
            q.push(*iter);
        }

        static int nx, ny;
        static int dx[] = {-1, 1, 0, 0};
        static int dy[] = {0, 0, -1, 1};

        while (!q.empty()) {
            count++;
            size_t len = q.size();
            for (size_t i = 0; i < len; i++) {
                pair<int, int> curr = q.front();
                q.pop();

                for (size_t k = 0; k < 4; k++) {
                    nx = curr.first + dx[k];
                    ny = curr.second + dy[k];
                    if (0 <= nx && nx < n && 0 <= ny && ny < n) {
                        if (grid[nx][ny] == 1) {
                            return count;
                        }

                        if (!vis[nx][ny]) {
                            vis[nx][ny] = true;
                            q.push(make_pair(nx, ny));
                        }
                    }
                }
            }
        }

        return count;
    }
};