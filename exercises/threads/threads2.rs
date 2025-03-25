// threads2.rs
//
// Building on the last exercise, we want all of the threads to complete their
// work but this time the spawned threads need to be in charge of updating a
// shared value: JobStatus.jobs_completed
//
// Execute `rustlings hint threads2` or use the `hint` watch subcommand for a
// hint.
use std::sync::{Arc, Mutex};
use std::thread;
use std::time::Duration;

struct JobStatus {
    jobs_completed: u32,
}

fn main() {
    // 将 JobStatus 包装在 Mutex 中，这样可以通过 Mutex 来管理对 jobs_completed 的修改
    let status = Arc::new(Mutex::new(JobStatus { jobs_completed: 0 }));
    let mut handles = vec![];

    for _ in 0..10 {
        let status_shared = Arc::clone(&status);
        let handle = thread::spawn(move || {
            thread::sleep(Duration::from_millis(250));
            // 使用 Mutex 来获取锁并修改共享数据
            let mut status_lock = status_shared.lock().unwrap();
            status_lock.jobs_completed += 1;
        });
        handles.push(handle);
    }

    // 等待所有线程完成工作
    for handle in handles {
        handle.join().unwrap();
    }

    // 打印最终的 jobs_completed 值
    let status_lock = status.lock().unwrap();
    println!("jobs completed {}", status_lock.jobs_completed);
}
