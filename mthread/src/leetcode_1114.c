// 法一：信号量

#include <stdio.h>
#include <semaphore.h>

typedef struct {
    sem_t sem_first_done;   // 通知 second 可执行
    sem_t sem_second_done;  // 通知 third 可执行    
} Foo;

// Function Declaration, do not remove
void printFirst();
void printSecond();
void printThird();

Foo* fooCreate() {
    Foo* obj = (Foo*) malloc(sizeof(Foo));
    // 第二个参数 0 表示线程间共享，第三个参数为初始值
    sem_init(&obj->sem_first_done, 0, 0);
    sem_init(&obj->sem_second_done, 0, 0);   
    return obj;
}

void first(Foo* obj) {
    printFirst();
    // V 操作：释放资源，唤醒等待的 second 线程
    sem_post(&obj->sem_first_done);
}

void second(Foo* obj) {
    // P 操作：等待 first 执行完成
    sem_wait(&obj->sem_first_done);
    printSecond();
    // V 操作：释放资源，唤醒等待的 third 线程
    sem_post(&obj->sem_second_done);
}

void third(Foo* obj) {
    // P 操作：等待 second 执行完成
    sem_wait(&obj->sem_second_done);
    printThird();
}

void fooFree(Foo* obj) {
    sem_destroy(&obj->sem_first_done);
    sem_destroy(&obj->sem_second_done);
    free(obj);
}