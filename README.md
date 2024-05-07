# 第二周：时空之门：从单线程到多线程，从同步到异步

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

## 使矩阵乘法支持多线程

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
