# 第二周：时空之门：从单线程到多线程，从同步到异步

## 主要代码

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

## 验证效果

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
