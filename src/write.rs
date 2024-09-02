use std::{
    fs::{File, OpenOptions},
    io::{Seek, SeekFrom},
    sync::mpsc::Receiver,
    thread,
};

pub struct AddressRecord {
    pub address: String,
    pub index: isize,
    pub address_type: String,
}

fn create_new_writer(path: Option<&str>, prefix: &str) -> Result<csv::Writer<File>, anyhow::Error> {
    // let timestamp = Local::now().format("%Y%m%d%H%M%S").to_string();
    let file_name = format!("{}_addresses.csv", prefix);
    // tracing::info!("path: {path:?}");
    let data_dir = if let Some(path) = path {
        std::path::Path::new(path).join("data")
    } else {
        std::path::Path::new("data").to_path_buf()
    };
    // tracing::info!("data_dir: {data_dir:?}");
    std::fs::create_dir_all(&data_dir)?;

    let file_path = data_dir.join(file_name);

    let file = OpenOptions::new()
        .create(true)
        .append(true)
        .open(file_path)?;
    Ok(csv::Writer::from_writer(file))
}

fn _check_file_size(writer: &mut csv::Writer<File>, max_size: u64) -> bool {
    let mut file = writer.get_ref();
    let file_size = file
        .seek(SeekFrom::End(0))
        .expect("Failed to get file size");
    file.seek(SeekFrom::Start(0))
        .expect("Failed to seek to start");
    file_size >= max_size
}

pub fn start_writer_thread(
    path: Option<String>,
    rx: Receiver<AddressRecord>,
    // max_file_size: u64,
    // rotation_interval: Duration,
) -> thread::JoinHandle<()> {
    thread::spawn(move || {
        if let Err(e) = do_writer(path, rx) {
            tracing::error!("do_writer error: {e:?}");
        }
    })
}

fn do_writer(path: Option<String>, rx: Receiver<AddressRecord>) -> Result<(), anyhow::Error> {
    let mut eth_writer = create_new_writer(path.as_deref(), "eth")?;
    let mut tron_writer = create_new_writer(path.as_deref(), "tron")?;
    // let start = Instant::now();

    for record in rx {
        // if start.elapsed() >= rotation_interval
        //     || check_file_size(&mut eth_writer, max_file_size)
        // {
        //     eth_writer = create_new_writer("eth");
        // }
        // if start.elapsed() >= rotation_interval
        //     || check_file_size(&mut tron_writer, max_file_size)
        // {
        //     tron_writer = create_new_writer("tron");
        // }

        match record.address_type.as_str() {
            "eth" => {
                eth_writer.write_record(&[record.address, record.index.to_string()])?;
            }
            "tron" => {
                tron_writer.write_record(&[record.address, record.index.to_string()])?;
            }
            _ => {}
        }

        eth_writer.flush()?;
        tron_writer.flush()?;
    }
    Ok(())
}
