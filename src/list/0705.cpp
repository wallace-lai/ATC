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

class HashBucket {
private:
    struct ListNode {
        unsigned int key;
        ListNode *next;

        ListNode() : key(0), next(nullptr) {}
        ListNode(unsigned int key) : key(key), next(nullptr) {}
    };

    size_t size;
    vector<ListNode> slot;

    ListNode *HashBucketFind(unsigned key) {
        size_t pos = key % size;
        ListNode *curr = &(slot[pos]);
        if (curr->key == 0) {
            return nullptr;
        }

        while (curr->next != nullptr) {
            if (curr->next->key == key) {
                return curr;
            }
            curr = curr->next;
        }

        return nullptr;
    };

public:
    HashBucket(size_t s) : size(s) { 
        slot.resize(s);
        for (size_t i = 0; i < s; i++) {
            slot[i].key = 0;
            slot[i].next = nullptr;
        }
    }

    void HashBucketAdd(unsigned key) {
        // ignore same item
        if (HashBucketFind(key) != nullptr) {
            return;
        }

        size_t pos = key % size;
        ListNode &dummy = slot[pos];

        ListNode *item = new ListNode(key);
        item->next = dummy.next;
        dummy.next = item;
        dummy.key++;
    }

    void HashBucketRemove(unsigned key) {
        ListNode *prev = HashBucketFind(key);
        if (prev == nullptr) {
            return;
        }

        ListNode *next = prev->next->next;
        ListNode *toDel = prev->next;
        prev->next = next;
        delete toDel;
    }

    bool HashBucketContains(unsigned key) {
        ListNode *prev = HashBucketFind(key);
        if (prev == nullptr) {
            return false;
        }

        return true;
    }
};

class MyHashSet {
private:
    HashBucket *bucket;

public:
    static const size_t BUCKET_SIZE = 997;

    MyHashSet() {
        bucket = new HashBucket(BUCKET_SIZE);
    }
    
    void add(int key) {
        bucket->HashBucketAdd(key);
    }
    
    void remove(int key) {
        bucket->HashBucketRemove(key);
    }
    
    bool contains(int key) {
        return bucket->HashBucketContains(key);
    }
};