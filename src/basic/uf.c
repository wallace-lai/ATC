// uf.c
// union and find set

#include <stdio.h>
#include <string.h>

#define MAX_NODE_SUM (128)
static unsigned sum;
static int pa[MAX_NODE_SUM];

void uf_init(unsigned n)
{
    assert(n <= MAX_NODE_SUM);

    sum = n;
    for (int i = 0; i < n; i++) {
        pa[i] = i;
    }
}

int uf_find0(int x)
{
    assert(x < sum);

    // no compress
    while (x != pa[x]) {
        x = pa[x];
    }

    return x;
}

int uf_find1(int x)
{
    assert(x < sum);

    // compress
    while (x != pa[x]) {
        pa[x] = pa[pa[x]];
        x = pa[x];
    }

    return x;
}

int uf_find2(int x)
{
    assert(x < sum);

    // full compress
    if (x != pa[x]) {
        pa[x] = uf_find2(pa[x]);
    }

    return pa[x];
}

typedef int (*uf_find_ptr)(int x);

int uf_union(int x, int y)
{
    uf_find_ptr uf_find = uf_find0;

    int xp = uf_find(x);
    int yp = uf_find(y);
    if (xp != yp) {
        pa[xp] = yp;
    }
}
