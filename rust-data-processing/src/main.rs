use std::time::Instant;
use sysinfo::{CpuExt, System, SystemExt};
use csv::ReaderBuilder; 
use rand::Rng;

fn count_words_from_text(text: &str) -> usize {
    let words: Vec<&str> = text.split_whitespace().collect();
    words.len()
}

fn count_words_from_csv(filename: &str) -> Result<usize, csv::Error> {
    let mut reader = ReaderBuilder::new().has_headers(true).from_path(filename)?; 

    let mut word_count = 0;
    for record in reader.records() {
        let record = record?;
        if let Some(review) = record.get(0) { // Assuming "review" is the first column
            word_count += count_words_from_text(review);
        }
    }

    Ok(word_count)
}

fn main() -> Result<(), csv::Error> {
    let filename = "../../movies.csv";

    let mut sys = System::new();
    let mut total_usage = 0.0;

    let start_time = Instant::now();

    let word_count = count_words_from_csv(filename)?;

    sys.refresh_all();
    for cpu in sys.cpus() {
        total_usage += cpu.cpu_usage();
    }
    std::thread::sleep(System::MINIMUM_CPU_UPDATE_INTERVAL);

    let end_time = Instant::now();
    let execution_time = end_time - start_time;
    let average_usage = total_usage / sys.cpus().len() as f32;

    println!("Count the number of words in a CSV file column.");
    println!("Word count: {:.1} Million", word_count as f32 / 1_000_000.0);
    println!("Time taken: {}.{:03} seconds", execution_time.as_secs(), execution_time.subsec_millis());
    println!("Average CPU core usage: {:.2}%", average_usage);
    println!("Memory usage: {} kilobytes", sys.total_memory() / 1000000);
    println!();

    // Generate and sort a list of 10,000,000 random integers
    let mut rng = rand::thread_rng(); 
    let mut integers: Vec<i32> = (0..10_000_000).map(|_| rng.gen_range(0..1_000_000)).collect();

    sys.refresh_all();
    total_usage = 0.0;

    let start_sort_time = Instant::now();
    integers.sort();
    let end_sort_time = Instant::now();
    let sort_execution_time = end_sort_time - start_sort_time;

    sys.refresh_all();
    for cpu in sys.cpus() {
        total_usage += cpu.cpu_usage();
    }
    std::thread::sleep(System::MINIMUM_CPU_UPDATE_INTERVAL);

    let average_usage_sort = total_usage / sys.cpus().len() as f32;

    println!("Generate and sort a list of 10,000,000 random integers.");
    println!("Time taken: {}.{:03} seconds", sort_execution_time.as_secs(), sort_execution_time.subsec_millis());
    println!("Average CPU core usage for sorting: {:.2}%", average_usage_sort);
    println!("Memory usage for sorting: {} kilobytes", sys.total_memory() / 1000000);

    Ok(())
}
