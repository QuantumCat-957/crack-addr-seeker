use std::sync::{atomic::Ordering, Arc};

use coins_bip32::xkeys::XPriv;

pub(crate) fn handle(
    key: XPriv,
    generator: impl crate::address::AddressGenerator + std::marker::Send + 'static,
    running: Arc<std::sync::atomic::AtomicBool>,
    generated_count: Arc<std::sync::atomic::AtomicUsize>,
    tx: Arc<std::sync::mpsc::Sender<crate::write::AddressRecord>>,
) -> Result<Vec<std::thread::JoinHandle<()>>, anyhow::Error> {
    let index_file_name = generator.index_file_name();
    let last_index = crate::address::read_last_index(&index_file_name)?;
    let handles: Vec<_> = (0..num_cpus::get() / 2)
        .map(|_| {
            let running = Arc::clone(&running);
            let generated_count = Arc::clone(&generated_count);
            let tx = Arc::clone(&tx);
            std::thread::spawn({
                let value = key.clone();
                {
                    let index_file_name = index_file_name.clone();
                    move || {
                        let mut index: u32 = last_index;

                        while running.load(Ordering::SeqCst) {
                            // 生成以太坊地址
                            let address = generator.generate_address(&value, index);
                            if let Ok(address) = address {
                                if crate::address::check_address(&address) {
                                    let record = crate::write::AddressRecord {
                                        address,
                                        index,
                                        address_type: generator.address_type(),
                                    };
                                    tx.send(record).expect("Failed to send address");
                                }
                                generated_count.fetch_add(1, Ordering::SeqCst);
                            }

                            index += 1;

                            if index % 100 == 0 {
                                crate::address::write_last_index(&index_file_name, index as u32)
                                    .ok();
                            }
                        }
                    }
                }
            })
        })
        .collect();

    Ok(handles)
}