pub mod eth;
pub mod trx;

pub trait AddressGenerator: Copy {
    fn generate_address(
        &self,
        key: &coins_bip32::xkeys::XPriv,
        index: isize,
    ) -> Result<String, anyhow::Error>;

    fn address_type(&self) -> String;
    fn index_file_name(&self) -> String;
}

pub fn check_address(address: &str, match_length: usize) -> bool {
    let len = address.len();
    address[len - match_length..]
        .chars()
        .all(|c| c == address.chars().nth(len - 1).unwrap())
}

pub fn write_last_index(
    path: Option<&str>,
    filename: &str,
    index: isize,
) -> Result<(), anyhow::Error> {
    let data_dir = if let Some(path) = path {
        std::path::Path::new(path).join("data")
    } else {
        std::path::Path::new("data").to_path_buf()
    };
    std::fs::create_dir_all(&data_dir).expect("Failed to create data directory");
    let file_path = data_dir.join(filename);
    tracing::info!("[write_last_index] file_path: {file_path:?}");
    std::fs::write(file_path, index.to_string())?;
    Ok(())
}

pub fn read_last_index(path: Option<&str>, filename: &str) -> Result<isize, anyhow::Error> {
    let data_dir = if let Some(path) = path {
        std::path::Path::new(path).join("data")
    } else {
        std::path::Path::new("data").to_path_buf()
    };
    std::fs::create_dir_all(&data_dir).expect("Failed to create data directory");
    let file_path = data_dir.join(filename);
    tracing::info!("[read_last_index] file_path: {file_path:?}");
    let content = std::fs::read_to_string(file_path).unwrap_or_else(|_| "0".to_string());
    let index = content.trim().parse().unwrap_or(0);
    Ok(index)
}
