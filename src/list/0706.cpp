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

struct BucketItem {
    int key;
    int value;
    BucketItem *next;

    BucketItem() : key(0), value(0), next(nullptr) {}
    BucketItem(int key, int value) : key(key), value(value), next(nullptr) {}
    BucketItem(int key, int value, BucketItem *next)
        : key(key), value(value), next(next) {}
};

class HashBucket {
private:
    size_t bucketSize;
    vector<BucketItem> slot;

    BucketItem *HashBucketFind(int key) {
        size_t pos = key % bucketSize;
        BucketItem *curr = &(slot[pos]);
        while (curr->next != nullptr) {
            if (curr->next->key == key) {
                return curr;
            }

            curr = curr->next;
        }

        return nullptr;
    }

public:
    HashBucket(size_t size) : bucketSize(size) {
        slot.resize(size);
    }

    void HashBucketPut(int key, int value) {
        BucketItem *prev = HashBucketFind(key);
        if (prev == nullptr) {
            BucketItem *newItem = new BucketItem(key, value);
            size_t pos = key % bucketSize;
            BucketItem &dummy = slot[pos];
            newItem->next = dummy.next;
            dummy.next = newItem;
            return;
        }

        prev->next->value = value;
        return;
    }

    int HashBucketGet(int key) {
        BucketItem *prev = HashBucketFind(key);
        if (prev == nullptr) {
            return -1;
        }

        return prev->next->value;
    }

    void HashBucketRemove(int key) {
        BucketItem *prev = HashBucketFind(key);
        if (prev == nullptr) {
            return;
        }

        BucketItem *toDel = prev->next;
        prev->next = prev->next->next;
        delete toDel;
        return;
    }
};

class MyHashMap {
private:
    unique_ptr<HashBucket> hash;

public:
    static const size_t BUCKET_SIZE = 7537;
    MyHashMap() {
        hash = make_unique<HashBucket>(BUCKET_SIZE);
    }
    
    void put(int key, int value) {
        hash->HashBucketPut(key, value);
    }
    
    int get(int key) {
        return hash->HashBucketGet(key);
    }
    
    void remove(int key) {
        hash->HashBucketRemove(key);
    }
};