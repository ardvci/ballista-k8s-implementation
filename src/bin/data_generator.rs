use arrow::array::*;
use arrow::record_batch::RecordBatch;
use parquet::arrow::arrow_writer::ArrowWriter;
use rand::Rng;
use std::fs;
use std::fs::File;
use std::sync::Arc;
use rusty_cloud::get_log_schema;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let schema = get_log_schema();
    let output_dir = "/mnt/";
    fs::create_dir_all(output_dir)?;

    let num_files = 40;
    let rows_per_file = 5_000_000;
    let batch_size = 1_000_000;
    let mut rng = rand::rng();

    println!("Starting generation of {} files, total ~200M rows...", num_files);

    for file_idx in 0..num_files {
        let file_path = format!("{}/logs_part_{}.parquet", output_dir, file_idx);
        let file = File::create(&file_path)?;
        let mut writer = ArrowWriter::try_new(file, schema.clone(), None)?;

        println!("Generating file: {}", file_path);

        for batch_idx in 0..(rows_per_file / batch_size) {
            let mut timestamps = Vec::with_capacity(batch_size);
            let mut user_ids = Vec::with_capacity(batch_size);
            let mut response_times = Vec::with_capacity(batch_size);
            let mut status_codes = Vec::with_capacity(batch_size);

            for _ in 0..batch_size {
                timestamps.push(1708527600 + (file_idx as i64 * rows_per_file as i64));
                user_ids.push(rng.random_range(1000..9999) as i64);
                response_times.push(rng.random_range(10.0..500.0));

                let status = if rng.random_bool(0.05) { 500 } else { 200 };
                status_codes.push(status);
            }

            let batch = RecordBatch::try_new(
                schema.clone(),
                vec![
                    Arc::new(Int64Array::from(timestamps)),
                    Arc::new(Int64Array::from(user_ids)),
                    Arc::new(Float64Array::from(response_times)),
                    Arc::new(Int32Array::from(status_codes)),
                ],
            )?;

            writer.write(&batch)?;
        }
        writer.close()?;
        println!("File {} completed.", file_idx + 1);
    }

    println!("Success! High-density dataset created in {}", output_dir);
    Ok(())
}