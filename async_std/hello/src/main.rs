use async_std::task;
use std::time::Duration;

async fn fetch_data() -> Result<String, reqwest::Error> {
    // Simulate fetching data asynchronously
    task::sleep(Duration::from_secs(2)).await;
    Ok("Data fetched successfully".to_string())
}

async fn process_data(data: String) -> String {
    // Simulate processing data asynchronously
    task::sleep(Duration::from_secs(1)).await;
    format!("Processed: {}", data)
}

async fn main_task() {
    // Fetch data asynchronously
    let data = fetch_data().await.expect("Failed to fetch data");

    // Process data asynchronously
    let processed_data = process_data(data).await;

    println!("{}", processed_data);
}

fn main() {
    // Run the asynchronous main task
    task::block_on(main_task());
}
