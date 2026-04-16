// Connect to Turso, create table, insert Ali with math=0, lang=0
// Run run_without_transaction() — two Tokio tasks simultaneously update math and lang, print final row, show the lost update
// Reset Ali back to math=0, lang=0
// Run run_with_transaction() — same two tasks but with transaction protection, print final row, show both updates survived
use std::sync::Arc;
use tokio::sync::Mutex;
use turso::Builder;
#[tokio::main]
async fn main() -> turso::Result<()> {
    // Create an in-memory database
    let db = Builder::new_local(":memory:").build().await?;
    let conn = db.connect()?;

    // Create a table
    conn.execute(
        "CREATE TABLE records (id INTEGER PRIMARY KEY, name TEXT, math INTEGER, lang INTEGER)",
        (),
    )
    .await?;

    // Insert data
    conn.execute(
        "INSERT INTO records (name, math, lang) VALUES (?1, ?2, ?3)",
        ["Rida", "70", "0"],
    )
    .await?;

    conn.execute(
        "INSERT INTO records (name, math, lang) VALUES (?1, ?2, ?3)",
        ["Shruti", "0", "0"],
    )
    .await?;

    // Query data
    let mut rows = conn.query("SELECT * FROM records", ()).await?;

    while let Some(row) = rows.next().await? {
        let id = row.get_value(0)?;
        let name = row.get_value(1)?;
        let math = row.get_value(2)?;
        let lang = row.get_value(3)?;
        println!(
            "User: {} - {} : ({},{})",
            id.as_integer().unwrap_or(&0),
            name.as_text().unwrap_or(&"".to_string()),
            math.as_integer().unwrap_or(&0),
            lang.as_integer().unwrap_or(&0)
        );
    }
    //-----------------------------------------------------------------------------------------------
    // old code begins
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
    //-----------------------------------------------------------------------------
    // old code ends
    //
    Ok(())
}
