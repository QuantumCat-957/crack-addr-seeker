use std::{fs::OpenOptions, sync::mpsc::Receiver, thread};
pub(crate) struct AddressRecord {
    pub(crate) address: String,
    pub(crate) index: u32,
    pub(crate) address_type: String,
}

pub fn start_writer_thread(rx: Receiver<AddressRecord>) -> thread::JoinHandle<()> {
    thread::spawn(move || {
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
    })
}
