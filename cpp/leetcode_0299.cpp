#include <vector>
#include <string>
#include <unordered_set>
#include <unordered_map>
using namespace std;

class Solution {
public:
    string getHint(string secret, string guess) {
        unordered_map<char, int> us;
        unordered_map<char, int> ug;

        size_t len = secret.size();
        for (size_t i = 0; i < len; i++) {
            us[secret[i]]++;
            ug[guess[i]]++;
        }

        int bulls = 0;
        decltype(secret.size()) i;
        for (i = 0; i < len; i++) {
            if (secret[i] == guess[i]) {
                bulls++;
                us[secret[i]]--;
                ug[guess[i]]--;
            }
        }

        int cows = 0;
        for (auto &i : us) {
            if (i.second > 0) {
                auto iter = ug.find(i.first);
                if (iter != ug.end() && iter->second > 0) {
                    int dec = min(i.second, iter->second);
                    us[i.first] -= dec;
                    ug[i.first] -= dec;
                    cows += dec;

                }
            }
        }

        return to_string(bulls) + "A" + to_string(cows) + "B";
    }
};