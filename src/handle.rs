use std::sync::{
    atomic::{AtomicUsize, Ordering},
    Arc,
};

use coins_bip32::xkeys::XPriv;

pub fn process(
    key: XPriv,
    generator: impl crate::address::AddressGenerator + std::marker::Send + 'static,
    running: Arc<std::sync::atomic::AtomicBool>,
    generated_count: Arc<std::sync::atomic::AtomicUsize>,
    tx: Arc<std::sync::mpsc::Sender<crate::write::AddressRecord>>,
    match_length: usize,
) -> Result<Vec<std::thread::JoinHandle<()>>, anyhow::Error> {
    let index_file_name = generator.index_file_name();
    let last_index = crate::address::read_last_index(&index_file_name)? as usize;
    let index_counter = Arc::new(AtomicUsize::new(last_index));
    let handles: Vec<_> = (0..num_cpus::get() / 2)
        .map(|_| {
            let running = Arc::clone(&running);
            let generated_count = Arc::clone(&generated_count);
            let tx = Arc::clone(&tx);
            std::thread::spawn({
                let value = key.clone();
                let index_file_name = index_file_name.clone();
                let index_counter = index_counter.clone();
                {
                    move || {
                        while running.load(Ordering::Acquire) {
                            let index = index_counter.load(Ordering::Relaxed) as u32;
                            let address = generator.generate_address(&value, index);
                            if let Ok(address) = address {
                                if crate::address::check_address(&address, match_length) {
                                    let record = crate::write::AddressRecord {
                                        address,
                                        index,
                                        address_type: generator.address_type(),
                                    };
                                    tx.send(record).expect("Failed to send address");
                                }
                                generated_count.fetch_add(1, Ordering::Relaxed);
                            }

                            if index % 100 == 0 {
                                crate::address::write_last_index(&index_file_name, index as u32)
                                    .ok();
                            }
                            // 获取并增加索引
                            index_counter.fetch_add(1, Ordering::Relaxed) as u32;
                        }
                    }
                }
            })
        })
        .collect();

    Ok(handles)
}
