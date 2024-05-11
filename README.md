# 第二周：时空之门：从单线程到多线程，从同步到异步

## Rust 并发

- 消息传递（Message passing）并发，其中 channel 被用来在线程间传递消息。
- 共享状态（Shared state）并发，其中多个线程可以访问同一片数据。
- Sync 和 Send trait，将 Rust 的并发保证扩展到用户定义的以及标准库提供的类型中。

## 简单支持 Rust 多线程

### 主要代码

在 Rust 中，move 关键字用于强制闭包获取其使用的环境值的所有权。这在闭包需要跨线程移动时尤其有用，因为这要求闭包是 'static（也就是说，它必须拥有其使用的所有值）。
producer 闭包在新线程中运行，它需要访问 i 和 tx。由于这两个变量都是来自外部环境的，所以我们需要使用 move 关键字来将它们的所有权转移到闭包中，这样它们就可以在新线程中安全地使用。
```rust
for i in 0..NUM_PRODUCERS {
    let tx = tx.clone();
    thread::spawn(move || producer(i, tx));
}
```

主线程会等待 consumer 线程完成，并尝试获取其返回的结果。
join 方法用于等待线程结束并获取其结果。这是一个阻塞调用，也就是说，调用 join 的线程会停止执行，直到目标线程完成。

```rust
let secret = consumer
    .join()
    .map_err(|e| anyhow!("Thread join error: {:?}", e))?;
```

### 验证效果

```bash
cargo run --example thread1

# 输出结果
consumer: Msg { idx: 3, value: 14272894328875654305 }
consumer: Msg { idx: 0, value: 14095101183812163186 }
consumer: Msg { idx: 2, value: 5605515674816571146 }
consumer: Msg { idx: 1, value: 14425188244267715876 }
producer 2 exit
consumer: Msg { idx: 0, value: 2090485248401438838 }
consumer: Msg { idx: 1, value: 6331331601848996684 }
consumer: Msg { idx: 3, value: 16496422359126968795 }
consumer: Msg { idx: 0, value: 3856554916000431448 }
consumer: Msg { idx: 1, value: 4684711684741873009 }
consumer: Msg { idx: 0, value: 14080192511636796303 }
producer 3 exit
consumer: Msg { idx: 0, value: 8650654798915285268 }
producer 1 exit
consumer: Msg { idx: 0, value: 9574458015338252395 }
producer 0 exit
consumer exit
secret: 77777
```

## 支持单线程矩阵乘法

矩阵乘法的基本原理是：结果矩阵 AB 中的每个元素，等于矩阵 A 的对应行与矩阵 B 的对应列的元素乘积之和。要进行矩阵乘法，A 的列数必须等于 B 的行数。结果矩阵 AB 的行数等于 A 的行数，列数等于 B 的列数。

具体来说，设 A 是 m×p 的矩阵，B 是 p×n 的矩阵，它们的乘积 AB 是 m×n 的矩阵。AB 中第 i 行第 j 列的元素（记为(AB)_ij）的计算公式为：

`(AB)_ij = A_i1 * B_1j + A_i2 * B_2j + ... + A_ip * B_pj`

其中，A_ik 表示矩阵 A 第 i 行第 k 列的元素，B_kj 表示矩阵 B第 k 行第 j 列的元素。

![](https://chengzw258.oss-cn-beijing.aliyuncs.com/Article/20240507071015.png)


### 主要代码

这段代码是在实现矩阵乘法运算：

1. 首先有三层嵌套的 for 循环，分别对应矩阵乘法中的三个维度:
    - 第一层循环变量 `i` 对应结果矩阵的行 (从 0 到 `a.row-1`)
    - 第二层循环变量 `j` 对应结果矩阵的列 (从 0 到 `b.col-1`)
    - 第三层循环变量 `k` 对应矩阵 a 的列，也是矩阵 b 的行 (从 0 到 `a.col-1`，注意矩阵 a 的列数需等于矩阵 b 的行数)

2. 在最内层循环中，进行实际的矩阵元素乘积求和:
   ```
   data[i*b.col + j] += a.data[i*a.col + k] * b.data[k*b.col + j]
   ```
    - `data` 是结果矩阵，`a` 和 `b` 是两个相乘的矩阵，`data[i*b.col + j]` 表示结果矩阵中第 i 行第 j 列的元素，该元素的值等于矩阵 a 第 i 行的所有元素与矩阵 b 第 j 列对应元素的乘积之和。
    - `a.data[i*a.col + k]` 表示矩阵 a 中第 i 行第 k 列的元素。
    - `b.data[k*b.col + j]` 表示矩阵 b 中第 k 行第 j 列的元素。

```rust
for i in 0..a.row {
    for j in 0..b.col {
        for k in 0..a.col {
            data[i * b.col + j] += a.data[i * a.col + k] * b.data[k * b.col + j]
        }
    }
}
```

为什么 a.data[i*a.col + k] 能表示矩阵 a 中第 i 行第 k 列的元素？

在这段代码中，矩阵是使用一维数组来存储的，而不是二维数组。为了在一维数组中表示二维矩阵，我们需要将二维的行列索引映射到一维数组的索引。

假设 a.col 是矩阵 a 的列数，那么 i*a.col 就是前 i 行的元素总数。接着加上 k，就得到了第 i 行第 k 列元素在一维数组中的索引。

举个例子，假设有一个 2x3 的矩阵，如下所示：

```
a = [[1, 2, 3],
     [4, 5, 6]]
```

假设这个矩阵以一维数组的形式存储在 `a.data` 中：`[1, 2, 3, 4, 5, 6]`。

1. 访问第 0 行第 1 列的元素（元素 `2`）：
   ```
   i = 0, k = 1
   a.data[0 * 3 + 1] = a.data[1] = 2
   ```

2. 访问第 1 行第 2 列的元素（元素 `6`）：
   ```
   i = 1, k = 2
   a.data[1 * 3 + 2] = a.data[5] = 6
   ```

## 使矩阵乘法支持多线程（消息传递并发）

### 添加依赖

Rust 的 oneshot 库提供了一种用于在异步编程中进行一次性通信的简单而高效的机制。它允许在异步任务之间发送单个值，非常适用于在任务之间传递结果或错误等一次性消息。

```rust
oneshot = "0.1.6"
```

### 主要代码

这段代码使用了多线程和消息传递来并行计算结果矩阵的每个元素。

1. 首先，它创建了一组发送者（`senders`），每个发送者都与一个新线程关联。每个线程都会接收消息，计算点积，并将结果发送回主线程。map 迭代器是惰性的，也就是说，它不会立即执行闭包并产生结果，而是在需要的时候才会执行。

```rust
let senders = (0..NUM_THREADS)
    .map(|_| {
        let (tx, rx) = mpsc::channel::<Msg<T>>();
        thread::spawn(move || {
            for msg in rx {
                let value = dot_product(msg.input.row, msg.input.col)?;
                if let Err(e) = msg.sender.send(MsgOutput {
                    idx: msg.input.idx,
                    value,
                }) {
                    eprintln!("Send error: {:?}", e);
                }
            }
            Ok::<_, anyhow::Error>(())
        });
        tx
    })
    .collect::<Vec<_>>();
```

2. 然后，它创建了一个接收者（`receivers`）的数组，用于接收每个线程的计算结果。同时，它也创建了一个数据（`data`）的数组，用于存储最终的矩阵乘法结果。

```rust
let matrix_len = a.row * b.col;
let mut data = vec![T::default(); matrix_len];
let mut receivers = Vec::with_capacity(matrix_len);
```

3. 接下来，它遍历输入矩阵的每一行和每一列，计算结果矩阵的每个元素。对于每个元素，它创建一个消息（`Msg`），包含了计算该元素所需的行和列，然后发送给一个线程。同时，它也创建了一个接收者（`rx`），用于接收线程的计算结果。

```rust
for i in 0..a.row {
    for j in 0..b.col {
        let row = Vector::new(&a.data[i * a.col..(i + 1) * a.col]);
        let col_data = b.data[j..]
            .iter()
            .step_by(b.col)
            .copied()
            .collect::<Vec<_>>();

        let col = Vector::new(col_data);
        let idx = i * b.col + j;
        let input = MsgInput::new(idx, row, col);
        let (tx, rx) = oneshot::channel();
        let msg = Msg::new(input, tx);
        if let Err(e) = senders[idx % NUM_THREADS].send(msg) {
            eprintln!("Send error: {:?}", e);
        }
        receivers.push(rx);
    }
}
```

4. 最后，它等待所有线程完成计算，并收集结果。每个线程的结果都会被存储在 `data` 数组的相应位置。

```rust
for rx in receivers {
    let output = rx.recv()?;
    data[output.idx] = output.value;
}
```

### 验证效果

```rust
cargo run --example matrix

a * b: {22 28, 49 64}
```


## 支持多线程修改 HashMap（共享内存）

### 主要代码

`data` 字段被包装在一个 `Mutex` 中，以实现线程安全。`Mutex` 是一个互斥锁，它保证了在任何时候只有一个线程可以访问 `data`。当一个线程正在访问 `data` 时，其他线程必须等待，直到该线程释放了锁。

然后，`Mutex<HashMap<String, i64>>` 被包装在一个 `Arc` 中。`Arc` 是一个原子引用计数类型，它允许多个线程共享 `data` 的所有权。当最后一个拥有 `data` 所有权的线程结束时，`data` 将被自动清理。

这个结构体实现了 `Clone` trait，这意味着你可以创建 `Metrics` 的副本。但是，由于 `data` 字段是 `Arc<Mutex<HashMap<String, i64>>>` 类型，所以克隆 `Metrics` 实际上只会增加 `data` 的引用计数，而不会复制 `data` 的数据。这样，即使 `Metrics` 被克隆了多次，多个 `Metrics` 实例也可以安全地共享同一份 `data` 数据。

```rust
#[derive(Debug, Clone)]
pub struct Metrics {
    data: Arc<Mutex<HashMap<String, i64>>>
}
```

这段代码是 `Metrics` 结构体的实现部分，它定义了三个方法：`new`，`inc` 和 `snapshot`。

- `new` 方法用于创建一个新的 `Metrics` 实例。它初始化了一个空的 `HashMap`，并将其包装在 `Arc<Mutex<HashMap<String, i64>>>` 中。
- `inc` 方法用于增加指定键的计数器。它首先获取 `data` 的锁，然后在 `HashMap` 中查找指定的键。如果找到了，就增加计数器的值；如果没有找到，就插入一个新的计数器。
- `snapshot` 方法用于获取当前的计数器快照。它首先获取 `data` 的锁，然后克隆 `HashMap`，并返回克隆的结果。
- 注意虽然我们在 `inc` 和 `snapshot` 方法中没有显式地释放锁，但当 `data` 的引用离开作用域时，锁会自动释放。

```rust
impl Metrics {
    pub fn new() -> Self {
        Metrics {
            data: Arc::new(Mutex::new(HashMap::new())),
        }
    }

    pub fn inc(&self, key: impl Into<String>) -> Result<()> {
        let mut data = self.data.lock().map_err(|e| anyhow!(e.to_string()))?;
        let counter = data.entry(key.into()).or_insert(0);
        *counter += 1;
        Ok(())
    }

    pub fn snapshot(&self) -> Result<HashMap<String, i64>> {
        Ok(self.data.lock().map_err(|e| anyhow!(e.to_string()))?.clone())
    }
}
```

分别创建两个线程池，执行不同的任务，`metrics.clone()` 是为了创建 `metrics` 的一个克隆，这样每个线程都可以拥有 `metrics` 的一个独立副本。

这是因为在 Rust 中，变量默认是移动语义（move semantics），也就是说，当你把一个变量传递给函数时，这个变量的所有权会被移动到函数中，原来的变量就不能再使用了。但在这里，我们需要在多个线程中共享 `metrics`，所以不能把 `metrics` 的所有权移动到一个线程中，而是需要创建 `metrics` 的克隆。

值得注意的是，虽然我们创建了 `metrics` 的克隆，但实际上 `metrics.data`（即 `Arc<Mutex<HashMap<String, i64>>>`）的数据并没有被复制。这是因为 `Arc`（Atomic Reference Counting，原子引用计数）类型的 `clone` 方法只会增加引用计数，而不会复制数据。这样，多个线程可以安全地共享同一份数据，而不需要复制数据

```rust
 for idx in 0..N {
     task_worker(idx, metrics.clone()); // Metrics {data: Arc::clone(&metrics.data)}
 }

 for _ in 0..M {
     request_worker(metrics.clone());
 }
```

### 验证效果

```bash
cargo run --example metrics

Ok({})
Ok({"req.page.4": 4, "req.page.3": 5, "req.page.2": 3, "req.page.1": 3, "call.thread.worker.0": 1, "call.thread.worker.1": 1})
Ok({"req.page.4": 9, "req.page.3": 9, "req.page.2": 7, "req.page.1": 9, "call.thread.worker.0": 2, "call.thread.worker.1": 1})
Ok({"req.page.4": 12, "req.page.3": 12, "req.page.2": 14, "req.page.1": 12, "call.thread.worker.0": 2, "call.thread.worker.1": 2})
Ok({"req.page.4": 15, "req.page.3": 18, "req.page.2": 20, "req.page.1": 16, "call.thread.worker.0": 3, "call.thread.worker.1": 2})
Ok({"req.page.4": 19, "req.page.3": 20, "req.page.2": 27, "req.page.1": 21, "call.thread.worker.0": 5, "call.thread.worker.1": 3})
Ok({"req.page.4": 26, "req.page.3": 28, "req.page.2": 32, "req.page.1": 25, "call.thread.worker.0": 5, "call.thread.worker.1": 3})
Ok({"req.page.4": 31, "req.page.3": 31, "req.page.2": 36, "req.page.1": 30, "call.thread.worker.0": 6, "call.thread.worker.1": 4})
Ok({"req.page.4": 34, "req.page.3": 40, "req.page.2": 39, "req.page.1": 33, "call.thread.worker.0": 6, "call.thread.worker.1": 5})
```

## 使用 RwLock 替换 Mutex

### 主要代码

使用 RwLock 代替 Mutex，RwLock 允许多个读取者同时访问数据，提高了并发性能。当有多个线程需要读取 HashMap 中的数据时，它们可以同时获取读锁，而无需相互阻塞。这对于读多写少的场景特别有利，因为读取操作不会相互干扰，可以并发进行。

```rust
#[derive(Debug, Clone)]
pub struct Metrics {
    data: Arc<RwLock<HashMap<String, i64>>>,
}
```

写方法使用 `write` 方法获取写锁，读方法使用 `read` 方法获取读锁。

```rust
 pub fn inc(&self, key: impl Into<String>) -> Result<()> {
     let mut data = self.data.write().map_err(|e| anyhow!(e.to_string()))?;
     let counter = data.entry(key.into()).or_insert(0);
     *counter += 1;
     Ok(())
 }

 pub fn snapshot(&self) -> Result<HashMap<String, i64>> {
     Ok(self
         .data
         .read()
         .map_err(|e| anyhow!(e.to_string()))?
         .clone())
 }
```

### 验证效果

```bash
cargo run --example metrics

req.page.4: 8
req.page.1: 4
req.page.2: 1
call.thread.worker.0: 1
req.page.3: 3

req.page.4: 13
call.thread.worker.1: 1
req.page.1: 9
req.page.2: 3
call.thread.worker.0: 2
req.page.3: 5
```

## 使用 DashMap 替换 RwLock 和 HashMap

DashMap 使用了一种称为锁分段（lock-striping）的技术，它将 HashMap 分成多个段，每个段都有自己的锁，进一步减少锁的粒度。这样，不同的线程可以同时访问不同的段，进一步提高了并发性能。

### 添加依赖

```rust
dashmap = "5.5.3"
```

### 主要代码

使用 DashMap 替换 RwLock 和 HashMap，DashMap 是一个线程安全的 HashMap，它允许多个线程同时读取和写入数据，而无需显式地获取锁。

```rust
#[derive(Debug, Clone)]
pub struct Metrics {
    data: Arc<DashMap<String, i64>>,
}
```

```rust
pub fn inc(&self, key: impl Into<String>) -> Result<()> {
  // 使用 DashMap 就不需要先获取锁了
  let mut counter = self.data.entry(key.into()).or_insert(0);
  *counter += 1;
  Ok(())
}
```

### 验证效果

```bash
cargo run --example metrics

req.page.2: 6
req.page.3: 4
req.page.4: 7
req.page.1: 4

req.page.2: 8
call.thread.worker.1: 1
req.page.3: 7
req.page.4: 15
req.page.1: 8
call.thread.worker.0: 1
```

## 使用 Atomic 原子类型

Mutex用起来简单，但是无法并发读，RwLock可以并发读，但是使用场景较为受限且性能不够，那么有没有一种全能性选手呢？ 欢迎我们的Atomic闪亮登场。

从 Rust1.34 版本后，就正式支持原子类型。原子指的是一系列不可被 CPU 上下文交换的机器指令，这些指令组合在一起就形成了原子操作。在多核 CPU 下，当某个 CPU 核心开始运行原子操作时，会先暂停其它 CPU 内核对内存的操作，以保证原子操作不会被其它 CPU 内核所干扰。

由于原子操作是通过指令提供的支持，因此它的性能相比锁和消息传递会好很多。相比较于锁而言，原子类型不需要开发者处理加锁和释放锁的问题，同时支持修改，读取等操作，还具备较高的并发性能，几乎所有的语言都支持原子类型。

可以看出原子类型是无锁类型，但是无锁不代表无需等待，因为原子类型内部使用了 CAS 循环，当大量的冲突发生时，该等待还是得等待！但是总归比锁要好。 CAS 全称是 Compare and swap, 它通过一条指令读取指定的内存地址，然后判断其中的值是否等于给定的前置值，如果相等，则将其修改为新的值


### 主要代码

使用 AtomicI64 原子整数类型，支持在多个线程之间安全地共享和修改一个 i64 类型的值，而无需使用互斥锁或其他同步机制。

```rust
#[derive(Debug)]
pub struct AmapMetrics {
    data: Arc<HashMap<&'static str, AtomicI64>>,
}
```

fetch_add 方法接收两个参数：要增加的值（在这里是 1）和一个内存顺序（在这里是 Ordering::Relaxed）。
Ordering::Relaxed 表示这个操作不需要与其他操作保持同步或顺序关系。这是最宽松的内存顺序，只保证了基本的原子性，不保证操作的顺序性。这意味着，在多线程环境中，不同线程看到的操作顺序可能会不同。
如果需要保证操作的顺序性，可以使用 Ordering::SeqCst（顺序一致性）。 参考：https://doc.rust-lang.org/std/sync/atomic/enum.Ordering.html

```rust
pub fn inc(&self, key: impl AsRef<str>) -> Result<()> {
  let key = key.as_ref();
  let counter = self
      .data
      .get(key)
      .ok_or_else(|| anyhow::anyhow!("key {} not found", key))?;
  counter.fetch_add(1, Ordering::Relaxed);
  Ok(())
}
```

### 验证效果

```bash
cargo run --example ametrics

call.thread.worker.0: 1
req.page.2: 7
call.thread.worker.1: 1
req.page.1: 2
req.page.4: 4
req.page.3: 4

call.thread.worker.0: 2
req.page.2: 13
call.thread.worker.1: 2
req.page.1: 6
req.page.4: 6
req.page.3: 9
```

## 参考资料

- [Rust 无畏并发](https://kaisery.github.io/trpl-zh-cn/ch16-00-concurrency.html)
- [透过 rust 探索系统的本原：并发篇](https://mp.weixin.qq.com/s/9g0wVT-5PpmXRoKJZo-skA)
- [线程同步：Atomic 原子类型与内存顺序](https://course.rs/advance/concurrency-with-threads/sync2.html)
