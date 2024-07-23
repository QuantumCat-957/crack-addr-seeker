mod address;
mod constant;
mod index;
mod language;
mod timer;
mod write;
mod xpriv;

use std::{
    sync::{
        atomic::{AtomicBool, AtomicUsize, Ordering},
        mpsc::{self},
        Arc,
    },
    thread,
};

use address::AddressGenerator;

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
    let (tx, rx) = mpsc::channel::<write::AddressRecord>();
    let tx = Arc::new(tx);

    let max_file_size = 100 * 1024 * 1024; // 100 MB
    let rotation_interval = std::time::Duration::from_secs(3600); // 1 hour
                                                                  // 启动一个线程来处理写入文件任务
    let writer_handle = write::start_writer_thread(rx, max_file_size, rotation_interval);

    // 启动计时器线程，每秒输出生成的地址数
    let timer_handle = timer::start_timer_thread(running_clone, generated_count_clone);

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
                let eth_gen = address::eth::EthereumAddressGenerator;
                move || {
                    let mut index: u32 = last_eth_index;

                    while running.load(Ordering::SeqCst) {
                        // 生成以太坊地址
                        let address = eth_gen.generate_address(&value, index);
                        if let Ok(address) = address {
                            if address::check_address(&address) {
                                let record = write::AddressRecord {
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
                let tron_gen = address::trx::TronAddressGenerator;
                move || {
                    let mut index: u32 = last_tron_index;
                    while running.load(Ordering::SeqCst) {
                        // 生成波场地址
                        let address = tron_gen.generate_address(&value, index);
                        if let Ok(address) = address {
                            if address::check_address(&address) {
                                let record = write::AddressRecord {
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
