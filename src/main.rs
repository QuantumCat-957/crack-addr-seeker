mod address;
mod constant;
mod language;
mod xpriv;

use std::{
    fs::OpenOptions,
    sync::{
        atomic::{AtomicBool, AtomicUsize, Ordering},
        mpsc::{self},
        Arc,
    },
    thread,
    time::{Duration, Instant},
};

struct AddressRecord {
    address: String,
    index: u32,
    address_type: String,
}

fn main() -> Result<(), anyhow::Error> {
    tracing_subscriber::fmt::init();

    let phrase = "fan swamp loop mesh enact tennis priority artefact canal hour skull joy";
    let bip39_pw = "123";

    let (key, _) = xpriv::phrase_to_master_key(1, &phrase, &bip39_pw)?;
    let running = Arc::new(AtomicBool::new(true));
    let running_clone = Arc::clone(&running);
    let generated_count = Arc::new(AtomicUsize::new(0));
    let generated_count_clone = Arc::clone(&generated_count);

    let eth_index_file = "eth_last_index.txt";
    let tron_index_file = "tron_last_index.txt";

    // 创建一个通道，用于发送地址写入任务
    let (tx, rx) = mpsc::channel::<AddressRecord>();
    let tx = Arc::new(tx);

    // 启动一个线程来处理写入文件任务
    let writer_handle = thread::spawn(move || {
        let eth_file = OpenOptions::new()
            .create(true)
            .append(true)
            .open("eth_addresses.csv")
            .expect("Failed to open file");
        let tron_file = OpenOptions::new()
            .create(true)
            .append(true)
            .open("tron_addresses.csv")
            .expect("Failed to open file");
        let mut eth_writer = csv::Writer::from_writer(eth_file);
        let mut tron_writer = csv::Writer::from_writer(tron_file);

        for record in rx {
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
        }
    });

    // 启动计时器线程，每秒输出生成的地址数
    let timer_handle = thread::spawn(move || {
        let start = Instant::now();
        let mut last_count = 0;
        while running_clone.load(Ordering::SeqCst) {
            thread::sleep(Duration::from_secs(1));
            let elapsed = start.elapsed().as_secs();
            if elapsed > 0 {
                let count = generated_count_clone.load(Ordering::SeqCst);
                let per_sec = count - last_count;
                last_count = count;
                tracing::info!("Addresses generated per second: {}", per_sec);
            }
        }
    });

    let last_eth_index = address::read_last_index(eth_index_file)?;
    let last_tron_index = address::read_last_index(tron_index_file)?;

    // 启动多个线程来生成地址
    let eth_handles: Vec<_> = (0..num_cpus::get() / 2)
        .map(|_| {
            let running = Arc::clone(&running);
            let generated_count = Arc::clone(&generated_count);
            let tx = Arc::clone(&tx);

            thread::spawn({
                let value = key.clone();
                move || {
                    let mut index: u32 = last_eth_index;

                    while running.load(Ordering::SeqCst) {
                        // 生成以太坊地址
                        let address = address::ethereum_address(&value, index);
                        if let Ok(address) = address {
                            if address::check_address(&address) {
                                let record = AddressRecord {
                                    address,
                                    index,
                                    address_type: "eth".to_string(),
                                };
                                tx.send(record).expect("Failed to send address");
                            }
                            generated_count.fetch_add(1, Ordering::SeqCst);
                        }

                        index += 1;

                        if index % 100 == 0 {
                            address::write_last_index(eth_index_file, index as u32).ok();
                        }
                    }
                }
            })
        })
        .collect();

    let tron_handles: Vec<_> = (0..num_cpus::get() / 2)
        .map(|_| {
            let running = Arc::clone(&running);
            let generated_count = Arc::clone(&generated_count);
            let tx = Arc::clone(&tx);

            thread::spawn({
                let value = key.clone();
                move || {
                    let mut index: u32 = last_tron_index;
                    while running.load(Ordering::SeqCst) {
                        // 生成波场地址
                        let address = address::tron_address(&value, index);
                        if let Ok(address) = address {
                            if address::check_address(&address) {
                                let record = AddressRecord {
                                    address,
                                    index,
                                    address_type: "tron".to_string(),
                                };
                                tx.send(record).expect("Failed to send address");
                            }
                            generated_count.fetch_add(1, Ordering::SeqCst);
                        }

                        index += 1;

                        if index % 100 == 0 {
                            address::write_last_index(tron_index_file, index as u32).ok();
                        }
                    }
                }
            })
        })
        .collect();

    for handle in eth_handles.into_iter().chain(tron_handles) {
        handle.join().expect("Thread failed");
    }

    running.store(false, Ordering::SeqCst);
    drop(tx); // 关闭发送端以终止接收线程
    writer_handle.join().expect("Writer thread failed");
    timer_handle.join().expect("Timer thread failed");

    Ok(())
}
