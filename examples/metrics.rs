use anyhow::Result;
use concurrency::Metrics;
use rand::Rng;
use std::{thread, time::Duration};

const N: usize = 2;
const M: usize = 4;

fn main() -> Result<()> {
    let metrics = Metrics::new();

    // start N task_worker and M request_worker
    println!("{}", metrics);

    for idx in 0..N {
        task_worker(idx, metrics.clone())?; // Metrics {data: Arc::clone(&metrics.data)}
    }

    for _ in 0..M {
        request_worker(metrics.clone())?;
    }

    loop {
        thread::sleep(Duration::from_secs(2));
        println!("{}", metrics);
    }
}

// 使用 Result<()> 作为函数的返回类型，明确表示函数可能返回错误。
// 这样可以提高代码的可读性和可维护性，让调用者清楚地知道函数可能会失败，并需要处理潜在的错误情况
fn task_worker(idx: usize, metrics: Metrics) -> Result<()> {
    thread::spawn(move || {
        loop {
            let mut rng = rand::thread_rng();
            thread::sleep(Duration::from_millis(rng.gen_range(100..5000)));
            metrics.inc(format!("call.thread.worker.{}", idx)).unwrap();
        }
        // 告诉编译器在这里忽略不可达代码的警告。这是因为在 loop 循环之后的代码实际上是不可达的，但是这个 Ok 语句是为了满足编译器对闭包返回类型的要求。
        #[allow(unreachable_code)]
        // ::<_, anyhow::Error> 是一个类型注释，用于指定 Ok 变体的类型参数。
        // 在这里，它表示 Ok 变体的值类型是 () (空元组)，而错误类型是 anyhow::Error。
        // _ 是一个类型占位符，表示编译器可以推断出实际的类型。在这种情况下，_代表 () 类型。
        Ok::<_, anyhow::Error>(())
    });
    Ok(())
}

fn request_worker(metrics: Metrics) -> Result<()> {
    thread::spawn(move || {
        loop {
            let mut rng = rand::thread_rng();
            thread::sleep(Duration::from_millis(rng.gen_range(50..800)));
            let page = rng.gen_range(1..5);
            metrics.inc(format!("req.page.{}", page)).unwrap();
        }
        #[allow(unreachable_code)]
        Ok::<_, anyhow::Error>(())
    });
    Ok(())
}
