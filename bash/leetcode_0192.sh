#!/usr/bin/env bash

# 法一：使用 awk 命令
# awk '{
#     for (i = 1; i <= NF; i++) {
#         count[$i]++
#     }
# }
# END {
#     for (word in count) {
#         print word, count[word]
#     }
# }' words.txt | sort -k2,2nr -k1,1

# 法二：tr + sort + uniq + awk
cat words.txt \
    | tr -s ' ' '\n' \
    | sort \
    | uniq -c \
    | sort -k1,1nr -k2,2 \
    | awk '{print $2, $1}'