use chrono::Local;
use flate2::write::GzEncoder;
use flate2::Compression;
use std::fs::{File, OpenOptions};
use std::io;
use std::io::Write;
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::Arc;
use std::thread;
use std::time::Duration;

const LOG_FILE: &str = "server.log";
const MAX_SIZE: u64 = 50 * 1024; // 50 KB

fn check_and_archive(filename: &str, current_size: u64, max_size: u64) -> bool {
    if current_size <= max_size {
        return false;
    }

    println!(
        "\n[ALERT] Log reached {} bytes. Rotating...",
        current_size
    );

    let now = Local::now();
    let timestamp = now.format("%Y-%m-%d_%H-%M-%S");
    let archive_name = format!("server-{}.log.gz", timestamp);

    let mut original = match File::open(filename) {
        Ok(f) => f,
        Err(e) => {
            println!("[ERROR] Could not open log file: {}", e);
            return false;
        }
    };

    let archive_file = match File::create(&archive_name) {
        Ok(f) => f,
        Err(e) => {
            println!("[ERROR] Could not create archive: {}", e);
            return false;
        }
    };

    let mut gz_writer = GzEncoder::new(archive_file, Compression::default());

    if let Err(e) = io::copy(&mut original, &mut gz_writer) {
        println!("[ERROR] Compression failed: {}", e);
        return false;
    }

    let archive_file = match gz_writer.finish() {
        Ok(f) => f,
        Err(e) => {
            println!("[ERROR] Could not finalize gzip: {}", e);
            return false;
        }
    };

    drop(archive_file);
    drop(original);

    println!("[ARCHIVE] Saved to: {}", archive_name);

    let file_to_clear = match OpenOptions::new().write(true).open(filename) {
        Ok(f) => f,
        Err(e) => {
            println!("[ERROR] Could not reopen log file: {}", e);
            return false;
        }
    };

    if let Err(e) = file_to_clear.set_len(0) {
        println!("[ERROR] Could not clear log file: {}", e);
        return false;
    }

    true
}

fn mock_server_worker(stop_flag: Arc<AtomicBool>) {
    let log_lines = vec![
        "INFO: User logged in. IP: 192.168.1.55\n",
        "INFO: Page requested. URL: /dashboard\n",
        "WARN: Slow query detected. Duration: 520ms\n",
        "INFO: User logged out. IP: 192.168.1.55\n",
        "WARN: High memory usage: 87%\n",
        "ERROR: Database connection timeout. Retrying...\n",
        "INFO: File uploaded successfully. Size: 2.4MB\n",
        "DEBUG: Cache miss for key: user_profile_42\n",
        "INFO: Password reset email sent to user@example.com\n",
        "FATAL: Payment gateway API unreachable.\n",
    ];

    let mut i = 0;

    while !stop_flag.load(Ordering::Relaxed) {
        if let Ok(mut f) = OpenOptions::new()
            .create(true)
            .append(true)
            .open(LOG_FILE)
        {
            let line = format!(
                "[{}] {}",
                Local::now().to_rfc3339(),
                log_lines[i % log_lines.len()]
            );

            let _ = f.write_all(line.as_bytes());
        }

        i += 1;
        thread::sleep(Duration::from_millis(10));
    }
}

fn main() {
    println!("=== LogRotator Engine Started ===");
    println!("Monitoring: {}", LOG_FILE);
    println!("Max allowed size: {} bytes (50 KB)", MAX_SIZE);
    println!("Press Ctrl+C to stop.");
    println!("---------------------------------\n");

    let stop_flag = Arc::new(AtomicBool::new(false));
    let stop_clone = Arc::clone(&stop_flag);

    ctrlc::set_handler(move || {
        stop_clone.store(true, Ordering::Relaxed);
    }).unwrap();


    thread::spawn(move || {
        mock_server_worker(stop_clone);
    });

    let mut rotation_count = 0;

    loop {
        if stop_flag.load(Ordering::Relaxed) {
            break;
        }

        thread::sleep(Duration::from_secs(2));

        let metadata = match std::fs::metadata(LOG_FILE) {
            Ok(m) => m,
            Err(_) => continue,
        };

        println!(
            "[MONITOR] server.log -> {} bytes",
            metadata.len()
        );

        if check_and_archive(LOG_FILE, metadata.len(), MAX_SIZE) {
            rotation_count += 1;
            println!(
                "[SUCCESS] Rotation #{} complete.\n",
                rotation_count
            );
        }
    }

    println!("---------------------------------");
    println!("LogRotator stopped. Total rotations: {}", rotation_count);
}
