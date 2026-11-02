#!/usr/bin/env bash

# 法一：使用 awk 命令
awk '
{
    # 遍历当前行的每一列
    for (i = 1; i <= NF; i++) {
        if (NR == 1) {
            # 如果是第一行，直接作为该列对应结果数组的初始值
            res[i] = $i
        } else {
            # 后续行，将字段追加到对应数组元素后，用空格分隔
            res[i] = res[i] " " $i
        }
    }
}
END {
    # 文件处理完毕后，按列的顺序输出数组的每个元素（即转置后的行）
    for (i = 1; i <= NF; i++) {
        print res[i]
    }
}' file.txt

# 法二：wc + awk + xargs
# 思路：先数出有多少列，然后依次提取每一列，再用 xargs 将其平铺

# # 获取第一行的列数
# COLS=$(head -1 file.txt | wc -w)

# # 循环处理每一列
# for ((i = 1; i <= COLS; i++)); do
#     # 用 awk 打印第 i 列的所哟值，通过管道传给 xargs
#     # xargs 默认会将多行输入合并成单行输出，恰好实现了转置
#     awk -v col=$i '{print $col}' file.txt | xargs
# done
