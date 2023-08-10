#include <string>
#include <vector>
#include <iostream>
using namespace std;

class Solution {
public:
    string mergeAlternately(string word1, string word2) {
        size_t len1 = word1.length();
        size_t len2 = word2.length();
        string ret(len1 + len2, '0');

        int i = 0;
        int t1, t2;
        for (t1 = 0, t2 = 0; t1 < len1 && t2 < len2; t1++, t2++) {
            ret[i++] = word1[t1];
            ret[i++] = word2[t2]; 
        }

        while (t1 < len1) {
            ret[i++] = word1[t1];
            t1++;
        }
        while (t2 < len2) {
            ret[i++] = word2[t2];
            t2++;
        }

        return ret;
    }
};