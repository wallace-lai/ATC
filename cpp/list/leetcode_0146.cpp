#include <set>
#include <map>
#include <stack>
#include <queue>
#include <string>
#include <vector>
#include <sstream>
#include <iostream>
#include <algorithm>
#include <unordered_map>
#include <unordered_set>
using namespace std;

struct DLinkedList {
    int key;
    int value;
    DLinkedList *prev;
    DLinkedList *next;

    DLinkedList() : key(0), value(0), prev(nullptr), next(nullptr) {}
    DLinkedList(int k, int v) : key(k), value(v), prev(nullptr), next(nullptr) {}
};

class LRUCache {
private:
    unordered_map<int, DLinkedList*> cache;
    DLinkedList head;
    DLinkedList tail;
    size_t size;
    size_t capacity;

public:
    LRUCache(int capacity) : capacity(capacity), size(0) {
        head.next = &tail;
        tail.prev = &head;
    }

    void AddToHead(DLinkedList *node) {
        node->prev = &head;
        node->next = head.next;
        head.next->prev = node;
        head.next = node;
    }

    void RemoveNode(DLinkedList *node) {
        node->next->prev = node->prev;
        node->prev->next = node->next;
    }

    void MoveToHead(DLinkedList *node) {
        RemoveNode(node);
        AddToHead(node);
    }

    DLinkedList *RemoveTail() {
        DLinkedList *node = tail.prev;
        RemoveNode(node);
        return node;
    }

    int get(int key) {
        if (cache.count(key) == 0) {
            return -1;
        }

        DLinkedList *node = cache[key];
        MoveToHead(node);
        return node->value;
    }


    void put(int key, int value) {
        if (cache.count(key) == 0) {
            DLinkedList *node = new DLinkedList(key, value);
            cache[key] = node;
            AddToHead(node);
            size++;
            if (size > capacity) {
                DLinkedList *toRemove = RemoveTail();
                cache.erase(toRemove->key);
                delete toRemove;
                size--;
            }

            return;
        }

        DLinkedList *node = cache[key];
        node->value = value;
        MoveToHead(node);
    }
};