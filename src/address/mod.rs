pub(crate) mod eth;
pub(crate) mod trx;

pub trait AddressGenerator: Copy {
    fn generate_address(
        &self,
        key: &coins_bip32::xkeys::XPriv,
        index: u32,
    ) -> Result<String, anyhow::Error>;

    fn address_type(&self) -> String;
    fn index_file_name(&self) -> String;
}

pub(crate) fn check_address(address: &str, match_length: usize) -> bool {
    let len = address.len();
    address[len - match_length..]
        .chars()
        .all(|c| c == address.chars().nth(len - 1).unwrap())
}

pub(crate) fn write_last_index(filename: &str, index: u32) -> Result<(), anyhow::Error> {
    let data_dir = std::path::Path::new("data");
    std::fs::create_dir_all(data_dir).expect("Failed to create data directory");
    let file_path = data_dir.join(filename);
    std::fs::write(file_path, index.to_string())?;
    Ok(())
}

pub(crate) fn read_last_index(filename: &str) -> Result<u32, anyhow::Error> {
    let data_dir = std::path::Path::new("data");
    std::fs::create_dir_all(data_dir).expect("Failed to create data directory");
    let file_path = data_dir.join(filename);
    let content = std::fs::read_to_string(file_path).unwrap_or_else(|_| "0".to_string());
    let index: u32 = content.trim().parse().unwrap_or(0);
    Ok(index)
}
