use chrono::Local;
use std::{
    fs::{File, OpenOptions},
    io::{Seek, SeekFrom},
    sync::mpsc::Receiver,
    thread,
    time::{Duration, Instant},
};

pub(crate) struct AddressRecord {
    pub(crate) address: String,
    pub(crate) index: u32,
    pub(crate) address_type: String,
}

fn create_new_writer(prefix: &str) -> csv::Writer<File> {
    let timestamp = Local::now().format("%Y%m%d%H%M%S").to_string();
    let file_name = format!("{}_addresses_{}.csv", prefix, timestamp);
    let file = OpenOptions::new()
        .create(true)
        .append(true)
        .open(file_name)
        .expect("Failed to open file");
    csv::Writer::from_writer(file)
}

fn check_file_size(writer: &mut csv::Writer<File>, max_size: u64) -> bool {
    let mut file = writer.get_ref();
    let file_size = file
        .seek(SeekFrom::End(0))
        .expect("Failed to get file size");
    file.seek(SeekFrom::Start(0))
        .expect("Failed to seek to start");
    file_size >= max_size
}

pub fn start_writer_thread(
    rx: Receiver<AddressRecord>,
    max_file_size: u64,
    rotation_interval: Duration,
) -> thread::JoinHandle<()> {
    thread::spawn(move || {
        let mut eth_writer = create_new_writer("eth");
        let mut tron_writer = create_new_writer("tron");
        let start = Instant::now();

        for record in rx {
            if start.elapsed() >= rotation_interval
                || check_file_size(&mut eth_writer, max_file_size)
            {
                eth_writer = create_new_writer("eth");
            }
            if start.elapsed() >= rotation_interval
                || check_file_size(&mut tron_writer, max_file_size)
            {
                tron_writer = create_new_writer("tron");
            }

            match record.address_type.as_str() {
                "eth" => {
                    eth_writer
                        .write_record(&[record.address, record.index.to_string()])
                        .expect("Failed to write to file");
                }
                "tron" => {
                    tron_writer
                        .write_record(&[record.address, record.index.to_string()])
                        .expect("Failed to write to file");
                }
                _ => {}
            }

            eth_writer.flush().expect("Failed to flush eth writer");
            tron_writer.flush().expect("Failed to flush tron writer");
        }
    })
}
