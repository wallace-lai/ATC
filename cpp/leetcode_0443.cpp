#include <string>
#include <cstring>
#include <vector>
#include <iostream>
using namespace std;

class Solution {
public:
    int compress(vector<char>& chars) {
        int idx = 0;
        int beg = 0;
        int end = 0;
        string num;

        size_t size = chars.size();
        while (end < size) {
            while (chars[end] == chars[beg]) {
                end++;
            }
            chars[idx++] = chars[beg];
            if (end - beg >= 2) {
                num = to_string(end - beg);
                for (auto &c : num) {
                    chars[idx++] = c;
                }
            }

            beg = end;
        }

        return idx;
    }
};

int main()
{
    // vector<char> vec = {
    //     'a', 'a', 'b', 'b', 'c', 'c', 'c'
    // };

    // vector<char> vec = { 'a' };

    vector<char> vec = {
        'a', 'b', 'b', 'b', 'b', 'b', 'b', 'b', 'b', 'b', 'b', 'b'
    };

    Solution s;
    int len = s.compress(vec);
    cout << "len = " << len << endl;
    for (int i = 0; i < len; i++) {
        cout << vec[i] << " ";
    }
    cout << endl;

    return 0;
}