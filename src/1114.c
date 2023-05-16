/**
 * Source : https://leetcode.cn/problems/print-in-order/
 *
 * Tag :
 * (1) multi-thread
 */

#include <semaphore.h>

/**
 * V1
 * Time : 	O(1)
 * Space : 	O(1)
 *
 * Time : 	56ms 93.70%
 * Space : 	6.4M 62.99%
 */

typedef struct {
    sem_t firstJobDone;
    sem_t secondJobDone;
} Foo;

Foo* fooCreate() {
    Foo* obj = (Foo*) malloc(sizeof(Foo));
    
    (void)sem_init(&obj->firstJobDone, 0, 0);
    (void)sem_init(&obj->secondJobDone, 0, 0);
    return obj;
}

void first(Foo* obj) {
    printFirst();
    sem_post(&obj->firstJobDone);
}

void second(Foo* obj) {
    sem_wait(&obj->firstJobDone);
    printSecond();
    sem_post(&obj->secondJobDone);
}

void third(Foo* obj) {
    sem_wait(&obj->secondJobDone);
    printThird();
}

void fooFree(Foo* obj) {
    (void)sem_destroy(&obj->firstJobDone);
    (void)sem_destroy(&obj->secondJobDone);
}