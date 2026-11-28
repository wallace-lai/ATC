# 线程同步

线程同步机制总结如下：

| 同步机制                        | 核心原理                                                     | 等待/阻塞方式                                         | 并发度与所有权                                        | 主要性能开销                                    | 主要优势                                       | 局限与注意事项                                               | 典型 C 接口                                                  | 典型场景                                    |
| :------------------------------ | :----------------------------------------------------------- | :---------------------------------------------------- | :---------------------------------------------------- | :---------------------------------------------- | :--------------------------------------------- | :----------------------------------------------------------- | :----------------------------------------------------------- | :------------------------------------------ |
| **互斥锁 Mutex**                | 通过加锁/解锁保证同一时刻只有一个线程进入临界区              | 获取失败时通常**睡眠阻塞**，锁释放后被唤醒            | 同一时刻仅 **1 个**线程持有；有明确持有者             | 竞争激烈时有**上下文切换**和内核/用户态切换开销 | 简单通用；等待时不浪费 CPU；适合保护较长临界区 | 临界区极短时切换开销可能不划算；需防死锁、重复加锁           | `pthread_mutex_t`：`pthread_mutex_lock/unlock`               | 保护共享变量、链表、计数器等任意临界区      |
| **自旋锁 Spinlock**             | 获取失败时线程**忙等待**，循环检查锁是否可用                 | **不睡眠**，持续占用 CPU 自旋                         | 同一时刻仅 **1 个**线程持有；有持有者                 | 无上下文切换，但等待时**浪费 CPU**              | 多核上锁持有时间极短时响应快、开销低           | 单核通常不适用；锁持有时间长会严重浪费 CPU；可能优先级反转   | `pthread_spinlock_t`：`pthread_spin_lock/unlock`             | 多核环境下极短临界区，如简单标志位保护      |
| **信号量 Semaphore**            | 维护一个计数，`P/wait` 申请资源减 1，`V/post` 释放资源加 1；计数为 0 时等待 | 计数不足时**阻塞睡眠**，`post` 后唤醒                 | 计数可 **>1**，允许多个线程同时访问；**无所有权概念** | 阻塞/唤醒有调度开销；计数操作需原子保证         | 可限制并发数量；可用于线程间通知和顺序控制     | 使用不当易出错；无持有者概念，一个线程可 `post` 另一个线程等待的信号量 | `sem_t`：`sem_wait/sem_post`                                 | 资源池限流、生产者-消费者计数、按序执行通知 |
| **条件变量 Condition Variable** | 线程在条件不满足时等待；另一线程修改条件后发出通知           | `wait` 时**原子释放互斥锁并睡眠**，被唤醒后重新获取锁 | 本身不提供互斥；条件由共享状态决定                    | 等待时基本不耗 CPU；唤醒涉及调度开销            | 高效等待复杂条件变化；适合事件通知             | 必须用 `while` 循环检查条件，防**虚假唤醒**；通知前需正确加锁 | `pthread_cond_t` + `pthread_mutex_t`：`pthread_cond_wait/signal/broadcast` | 生产者-消费者模型、复杂状态等待             |
| **读写锁 Read-Write Lock**      | 区分读锁和写锁：读锁共享，写锁独占                           | 读锁可并发；写锁冲突时阻塞等待                        | 多读并发，或 **1 个写者独占**；读/写有持有状态        | 状态维护比互斥锁复杂；写频繁时可能不如互斥锁    | 读多写少场景可显著提高读并发                   | 实现可能有读者优先/写者优先差异；写饥饿风险需注意            | `pthread_rwlock_t`：`rdlock/wrlock/unlock`                   | 缓存、配置表等读多写少共享数据              |
| **原子操作 Atomic**             | 依赖 CPU 原子指令或 CAS，保证单个操作不可分割                | **无锁、不阻塞**                                      | 针对单个原子变量；无传统锁所有权                      | 通常极低；高竞争下 CAS 可能反复重试             | 无锁、性能高；适合简单计数和标志位             | 只适合单变量简单操作，不能保护复杂临界区；需注意内存序       | C11 `stdatomic.h`：`atomic_int`、`atomic_fetch_add` 等       | 计数器、状态标志、无锁数据结构基础          |
| **屏障 Barrier**                | 一组线程在屏障点相互等待，全部到达后一起继续                 | 先到线程**阻塞等待**，最后一个到达后集体释放          | 协调一组线程；无资源所有权                            | 涉及集体等待和唤醒；线程数多时协调开销增加      | 实现阶段性集体同步，逻辑清晰                   | 只适合“所有线程到齐再继续”的场景；需固定线程组               | `pthread_barrier_t`：`pthread_barrier_wait`                  | 分阶段并行计算、每阶段结束统一同步          |

## POSIX 接口

### 互斥锁

（1）创建与销毁

```c
#include <pthread.h>

int pthread_mutex_init(pthread_mutex_t *restrict mutex,
    const pthread_mutexattr_t *restrict attr);

pthread_mutex_t mutex = PTHREAD_MUTEX_INITIALIZER;

int pthread_mutex_destroy(pthread_mutex_t *mutex);
```

（2）加锁和解锁

```c
int pthread_mutex_lock(pthread_mutex_t *mutex);
int pthread_mutex_trylock(pthread_mutex_t *mutex);
int pthread_mutex_unlock(pthread_mutex_t *mutex);
```

### 自旋锁

（1）创建与销毁

```c
#include <pthread.h>

int pthread_spin_init(pthread_spinlock_t *lock, int pshared);
int pthread_spin_destroy(pthread_spinlock_t *lock);
```

（2）加锁和解锁

```c
int pthread_spin_lock(pthread_spinlock_t *lock);
int pthread_spin_trylock(pthread_spinlock_t *lock);
int pthread_spin_unlock(pthread_spinlock_t *lock);
```

### 读写锁

（1）创建与销毁

```c
#include <pthread.h>

int pthread_rwlock_init(pthread_rwlock_t *restrict rwlock,
    const pthread_rwlockattr_t *restrict attr);
pthread_rwlock_t rwlock = PTHREAD_RWLOCK_INITIALIZER;

int pthread_rwlock_destroy(pthread_rwlock_t *rwlock);
```

（2）加锁与解锁

```c
int pthread_rwlock_rdlock(pthread_rwlock_t *rwlock);
int pthread_rwlock_tryrdlock(pthread_rwlock_t *rwlock);

int pthread_rwlock_wrlock(pthread_rwlock_t *rwlock);
int pthread_rwlock_trywrlock(pthread_rwlock_t *rwlock);

int pthread_rwlock_unlock(pthread_rwlock_t *rwlock);
```

### 信号量

（1）创建与销毁

```c
#include <semaphore.h>

int sem_init(sem_t *sem, int pshared, unsigned int value);
int sem_destroy(sem_t *sem);
```

（2）P 操作

```c
int sem_wait(sem_t *sem);
int sem_trywait(sem_t *sem);
int sem_timedwait(sem_t *restrict sem,
    const struct timespec *restrict abs_timeout);
```

（3）V 操作

```c
int sem_post(sem_t *sem);
```

### 条件变量

（1）创建与销毁

```c
#include <pthread.h>

int pthread_cond_init(pthread_cond_t *restrict cond,
    const pthread_condattr_t *restrict attr);
pthread_cond_t cond = PTHREAD_COND_INITIALIZER;

int pthread_cond_destroy(pthread_cond_t *cond);
```

（2）阻塞

```c
int pthread_cond_wait(pthread_cond_t *restrict cond,
    pthread_mutex_t *restrict mutex);

int pthread_cond_timedwait(pthread_cond_t *restrict cond,
    pthread_mutex_t *restrict mutex,
    const struct timespec *restrict abstime);
```

（3）唤醒

```c
int pthread_cond_broadcast(pthread_cond_t *cond);
int pthread_cond_signal(pthread_cond_t *cond);
```

### 原子操作

（1）初始化

```c
#include <stdatomic.h>

void atomic_init( volatile A* obj, C desired );
```

（2）算术与逻辑运算

```c
atomic_fetch_add
atomic_fetch_add_explicit

atomic_fetch_sub
atomic_fetch_sub_explicit

atomic_fetch_and
atomic_fetch_and_explicit

atomic_fetch_or
atomic_fetch_or_explicit

atomic_fetch_xor
atomic_fetch_xor_explicit
```

（3）读取与交换

```c
atomic_store
atomic_store_explicit

atomic_load
atomic_load_explicit

atomic_exchange
atomic_exchange_explicit
```

（4）CAS原语

```c
atomic_compare_exchange_strong
atomic_compare_exchange_strong_explicit

atomic_compare_exchange_weak
atomic_compare_exchange_weak_explicit
```

（5）内存序

- memory_order_relaxed
- memory_order_consume
- memory_order_acquire
- memory_order_release
- memory_order_acq_rel
- memory_order_seq_cst


### 屏障

**`pthread` 屏障和内存序的关系：**屏障是一个强制的、粗粒度的内存同步原语。它不接收内存序参数，但其 POSIX 规范定义的语义，在实现上正是通过 acquire/release 内存序来达成的，从而在所有参与线程间建立起可靠的 happens-before 关系，保证屏障之前的内存操作对之后的所有线程可见。

（1）创建

```c
int pthread_barrier_init(pthread_barrier_t *restrict barrier,
    const pthread_barrierattr_t *restrict attr,
    unsigned count);
```

- 创建一个屏障对象，并指定参与同步的线程数量 `count`；
- 只有当 count 个线程都调用了 `pthread_barrier_wait` 后，所有线程才会被同时放行；

（2）销毁

```c
int pthread_barrier_destroy(pthread_barrier_t *barrier);
```
- 释放屏障占用的资源；
- 前提是没有任何线程正阻塞在该屏障上，否则行为未定义；
- 销毁后，屏障对象可以被重新 `init` 复用；

（2）在屏障上等待

```c
int pthread_barrier_wait(pthread_barrier_t *barrier);
```

- 调用此函数的线程会阻塞，直到等待的线程数达到初始化时指定的 `count`；
- 此时，所有等待的线程会同时被唤醒并继续执行；

这个函数的返回值有一个特殊设计：

- 对于每一批被同步的 count 个线程，有且仅有一个线程会收到特殊返回值 PTHREAD_BARRIER_SERIAL_THREAD。

- 其余所有线程的返回值为 0。

这个机制通常用于让某个线程（收到特殊值的那个）在屏障后执行一次性的“串行”工作（如汇总结果），而其他线程继续并行。具体哪个线程收到该值是不确定的。


# C11/POSIX 六种内存序详解

## 🧠 背景：为什么需要内存序

在理解内存序之前，必须先明白一个问题：**编译器和 CPU 都会对内存访问指令进行重排**。

- **编译器**为了优化，可能改变指令顺序。
- **CPU** 通过乱序执行、Store Buffer、缓存一致性协议等机制，使得一个线程的写操作以不同于程序顺序的方式被另一个线程观察到。

在单线程中这没有问题（as-if 规则保证结果一致），但在多线程中，如果没有约束，一个线程写入的数据可能以"错误"的顺序被另一个线程看到，导致数据竞争和逻辑错误。

**内存序（memory order）就是用来约束这种重排的**，它告诉编译器和 CPU：哪些重排是允许的，哪些必须禁止。

C11 标准定义了六种内存序，从最宽松到最严格依次是：

```
relaxed < consume < acquire < seq_cst   （读侧）
relaxed < release < seq_cst             （写侧）
acq_rel = acquire + release             （读-改-写）
```

---

## 1️⃣ `memory_order_relaxed`：最宽松

**语义**：只保证**原子性**和**该原子变量自身的修改顺序一致性**（modification order）。不提供任何跨线程的同步或顺序保证。

- 该原子操作本身是不可分割的。
- 所有线程对该变量的修改会形成一个全局一致的顺序（不会出现两个线程看到不同的修改顺序）。
- **但**：它前后的普通读写可以被任意重排到它之前或之后。

**典型用途**：单纯计数、统计，不需要用这个值去同步其他数据。

```c
// 引用计数增加，不需要任何顺序保证
atomic_fetch_add_explicit(&refcount, 1, memory_order_relaxed);
```

---

## 2️⃣ `memory_order_consume`：依赖排序（已不推荐）

**语义**：只对**数据依赖**的操作施加顺序约束。它保证：在该 consume 操作**之后**、且**依赖于该操作返回值**的读写，不会被重排到它之前。对没有数据依赖的操作，则不做约束。

**现状**：这是设计上最微妙、实现上最困难的一种内存序。由于编译器很难精确追踪"数据依赖"，几乎所有实现都直接将其**等同于 `memory_order_acquire`**。C++17 曾建议废弃，**C++20 正式将其标记为不推荐使用（deprecated）**，建议直接用 `acquire` 替代。POSIX 对齐 C 标准，因此也继承了这一状态。

**结论**：**实际编程中不要使用它，用 `acquire` 代替。**

---

## 3️⃣ `memory_order_acquire`：获取（用于读）

**语义**：用于 **load** 操作。保证在该 acquire 操作**之后**的所有读写操作，**不会被重排到该操作之前**。

它就像一个"向下单向闸门"：闸门之后的操作不能跑到闸门之前。

**核心作用**：与 `release` 配对，建立 **synchronizes-with**（同步于）关系。

```
线程 A:  store(data)  →  store(flag, release)
线程 B:  load(flag, acquire)  →  load(data)
```

如果 B 的 acquire load 读到了 A 的 release store 所写的值，那么 A 在 release 之前的**所有写入**（包括 `data`）对 B 在 acquire 之后的**所有读取**都是可见的。这就是经典的**发布-订阅（publish-subscribe）**模式。

---

## 4️⃣ `memory_order_release`：释放（用于写）

**语义**：用于 **store** 操作。保证在该 release 操作**之前**的所有读写操作，**不会被重排到该操作之后**。

它就像一个"向上单向闸门"：闸门之前的操作不能跑到闸门之后。

与 `acquire` 配合，确保"发布"操作之前的所有数据修改，在对端"获取"之后都可见。

---

## 5️⃣ `memory_order_acq_rel`：获取-释放（用于读-改-写）

**语义**：同时具备 `acquire` 和 `release` 的语义。用于**读-改-写（RMW）**操作，如 `atomic_fetch_add`、`atomic_compare_exchange`、`atomic_exchange` 等。

- 作为"读"（看到旧值）时，有 acquire 语义；
- 作为"写"（写入新值）时，有 release 语义。

**典型场景**：自旋锁的 `compare_exchange`、无锁队列中的节点交换、引用计数的"最后一个释放者"判断。

```c
// 自旋锁加锁
while (!atomic_exchange_explicit(&lock, 1, memory_order_acq_rel)) { /* spin */ }
```

---

## 6️⃣ `memory_order_seq_cst`：顺序一致性（最严格）

**语义**：最强的一致性保证。除了具备 acquire/release 的全部语义外，还额外保证：**所有 `seq_cst` 操作在所有线程之间形成一个单一的全局全序（total order）**，每个线程看到的这些操作的先后顺序都是一致的。

这是 C11 的**默认内存序**（所有不带 `_explicit` 的原子操作都隐含使用它）。它最容易推理，但性能开销也最大（在弱内存模型架构如 ARM/PowerPC 上可能需要内存屏障指令）。

**典型场景**：Dekker 算法、Peterson 算法、需要全局一致顺序的复杂同步协议。

---

## 📊 强弱关系与归类

| 内存序 | 适用操作 | 关键保证 | 与 Pthreads 的类比 |
|---|---|---|---|
| `relaxed` | load/store/RMW | 仅原子性与修改顺序 | 无 |
| `consume` | load | 仅依赖排序（已弃用） | ≈ acquire |
| `acquire` | load | 之后的读不能前移 | `pthread_mutex_lock` |
| `release` | store | 之前的读不能后移 | `pthread_mutex_unlock` |
| `acq_rel` | RMW | acquire + release | lock 与 unlock 合一 |
| `seq_cst` | 全部 | 全局单一全序 | 最强的 mutex 语义 |

**包含关系**：

- `seq_cst` ⊃ `acquire` ⊃ `consume` ⊃ `relaxed`（读侧）
- `seq_cst` ⊃ `release` ⊃ `relaxed`（写侧）
- `acq_rel` = `acquire` ∪ `release`

---

## 🎯 实践中的选择建议

1. **默认用 `seq_cst`**：除非有明确的性能需求，否则它最不容易出错。
2. **发布-订阅用 `release`/`acquire`**：这是最常见的高性能模式（如初始化数据后发布指针）。
3. **纯计数用 `relaxed`**：引用计数、统计计数器不需要顺序保证。
4. **RMW 操作用 `acq_rel`**：如自旋锁、无锁数据结构中的 CAS。
5. **永远不要用 `consume`**：已被标准弃用，用 `acquire` 替代。

### 💡 一个经典的心智模型

把 `acquire` 想成"**读闸门向下开**"（后面的操作不能跑到前面），把 `release` 想成"**写闸门向上开**"（前面的操作不能跑到后面）。两者配对时，release 之前的所有写入，都会被 acquire 之后的所有读取所看到——这就是**happens-before** 关系的建立方式，也是所有无锁同步正确性的根基。
