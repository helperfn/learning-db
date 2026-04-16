// Connect to Turso, create table, insert Ali with math=0, lang=0
// Run run_without_transaction() — two Tokio tasks simultaneously update math and lang, print final row, show the lost update
// Reset Ali back to math=0, lang=0
// Run run_with_transaction() — same two tasks but with transaction protection, print final row, show both updates survived

use std::sync::Arc;
use tokio::sync::Mutex;

#[tokio::main]
async fn main() {
    let counter = Arc::new(Mutex::new(0));
    let c1 = Arc::clone(&counter);
    let c2 = Arc::clone(&counter);

    let thread1 = tokio::spawn(async move {
        for _ in 0..5 {
            let mut num = c1.lock().await;
            *num += 1;
            println!("number after thread 1 operation: {}", *num);
        }
        "thread 1 done"
    });

    let thread2 = tokio::spawn(async move {
        for _ in 0..5 {
            let mut num = c2.lock().await;
            *num += 10;
            println!("number after thread 2 operation: {}", *num);
        }
        "thread 2 done"
    });

    let r1 = thread1.await.unwrap();
    let r2 = thread2.await.unwrap();

    println!("--- Results ---");
    println!("{}", r1);
    println!("{}", r2);

    println!("Final value: {}", *counter.lock().await);
}
