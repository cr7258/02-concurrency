use anyhow::{anyhow, Result};
use std::{sync::mpsc, thread, time::Duration};

const NUM_PRODUCERS: usize = 4;

// 因为我们在代码中没有实际使用到 Msg 的所有字段，所以这里加上 #[allow(dead_code)] 来消除警告
#[allow(dead_code)]
#[derive(Debug)]
struct Msg {
    idx: usize,
    value: usize,
}

fn main() -> Result<()> {
    let (tx, rx) = mpsc::channel();

    // 创建 producer
    for i in 0..NUM_PRODUCERS {
        let tx = tx.clone();
        thread::spawn(move || producer(i, tx));
    }
    // 因为 producer 的 tx 是 clone 的，所以这里需要 drop 最初的 tx
    drop(tx);

    // 创建 consumer
    let consumer = thread::spawn(move || {
        for msg in rx {
            println!("consumer: {:?}", msg);
        }
        println!("consumer exit");
        77777 // a secret number
    });

    // 主线程会等待 consumer 线程完成，并尝试获取其返回的结果。
    // join 方法用于等待线程结束并获取其结果。
    // 这是一个阻塞调用，也就是说，调用 join 的线程会停止执行，直到目标线程完成。

    // map_err 方法用于将原始的 Err 值转换为 anyhow 错误。
    // 这里的 |e| anyhow!("Thread join error: {:?}", e) 是一个闭包，它接受一个参数 e（原始的 Err 值），并返回一个新的 anyhow 错误，其中包含了原始错误的信息。
    let secret = consumer
        .join()
        .map_err(|e| anyhow!("Thread join error: {:?}", e))?;

    println!("secret: {}", secret);

    Ok(())
}

fn producer(idx: usize, tx: mpsc::Sender<Msg>) -> Result<()> {
    loop {
        let value = rand::random::<usize>();
        tx.send(Msg::new(idx, value))?;
        let sleep_time = rand::random::<u8>() as u64 * 10;
        thread::sleep(Duration::from_millis(sleep_time));
        // random exit the producer
        if rand::random::<u8>() % 5 == 0 {
            println!("producer {} exit", idx);
            break;
        }
    }
    Ok(())
}

impl Msg {
    fn new(idx: usize, value: usize) -> Self {
        Self { idx, value }
    }
}
