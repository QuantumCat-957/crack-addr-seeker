use std::sync::{
    atomic::{AtomicIsize, Ordering},
    Arc,
};

use coins_bip32::xkeys::XPriv;

pub fn process(
    path: Option<String>,
    key: XPriv,
    generator: impl crate::address::AddressGenerator + std::marker::Send + 'static,
    running: Arc<std::sync::atomic::AtomicBool>,
    generated_count: Arc<std::sync::atomic::AtomicIsize>,
    tx: Arc<std::sync::mpsc::Sender<crate::write::AddressRecord>>,
    match_length: usize,
    reverse_index: bool,
) -> Result<Vec<std::thread::JoinHandle<()>>, anyhow::Error> {
    let index_file_name = generator.index_file_name();
    let last_index = crate::address::read_last_index(path.as_deref(), &index_file_name)?;

    let index_counter = Arc::new(AtomicIsize::new(last_index));
    let mut cpus = num_cpus::get();
    if cpus <= 1 {
        cpus = 16
    }
    tracing::info!("cpus: {}", num_cpus::get() / 2);
    let handles: Vec<_> = (0..cpus)
        .map(|_| {
            let running = Arc::clone(&running);
            let generated_count = Arc::clone(&generated_count);
            let tx = Arc::clone(&tx);
            std::thread::spawn({
                let value = key.clone();
                let index_file_name = index_file_name.clone();
                let index_counter = index_counter.clone();
                let path = path.clone();
                {
                    move || {
                        while running.load(Ordering::Acquire) {
                            let index = index_counter.load(Ordering::Relaxed);
                            let address = generator.generate_address(&value, index);
                            if let Ok(address) = address {
                                if crate::address::check_address(&address, match_length) {
                                    let record = crate::write::AddressRecord {
                                        address,
                                        index,
                                        address_type: generator.address_type(),
                                    };
                                    if let Err(res) = tx.send(record) {
                                        tracing::error!("Failed to send address: {res}")
                                    };
                                }
                                if reverse_index {
                                    generated_count.fetch_sub(1, Ordering::Relaxed);
                                } else {
                                    generated_count.fetch_add(1, Ordering::Relaxed);
                                }
                            }

                            if index % 100 == 0 {
                                let _ = crate::address::write_last_index(
                                    path.as_deref(),
                                    &index_file_name,
                                    index,
                                );
                                // tracing::error!("[write_last_index] res: {res:?}");
                            }
                            // 获取并增加索引
                            if reverse_index {
                                index_counter.fetch_sub(1, Ordering::Relaxed);
                            } else {
                                index_counter.fetch_add(1, Ordering::Relaxed);
                            }
                        }
                    }
                }
            })
        })
        .collect();

    Ok(handles)
}
