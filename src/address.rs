use anychain_core::Address as _;

pub(crate) fn ethereum_address(
    key: &coins_bip32::xkeys::XPriv,
    index: u32,
) -> Result<String, anyhow::Error> {
    let path = crate::constant::add_index(crate::constant::ETH_DERIVATION_PATH, index);
    let derive = key.derive_path(path.as_str())?;
    let signingkey: &coins_bip32::ecdsa::SigningKey = derive.as_ref();

    let address = alloy::signers::utils::secret_key_to_address(signingkey);
    Ok(address.to_string())
}

pub(crate) fn tron_address(
    key: &coins_bip32::xkeys::XPriv,
    index: u32,
) -> Result<String, anyhow::Error> {
    let path = crate::constant::add_index(crate::constant::TRON_DERIVATION_PATH, index);
    let derive = key.derive_path(path.as_str())?;
    let signingkey: &coins_bip32::ecdsa::SigningKey = derive.as_ref();
    let private_key = signingkey.to_bytes();

    let private_key = libsecp256k1::SecretKey::parse_slice(&private_key)?;
    let address = anychain_tron::TronAddress::from_secret_key(
        &private_key,
        &anychain_tron::TronFormat::Standard,
    )
    .unwrap();
    Ok(address.to_string())
}

pub(crate) fn check_address(address: &str) -> bool {
    let len = address.len();
    address[len - 2..]
        .chars()
        .all(|c| c == address.chars().nth(len - 1).unwrap())
}

// pub(crate) fn save_address_with_writer(
//     address: &str,
//     index: u32,
//     writer: &mut BufWriter<File>,
// ) -> Result<(), anyhow::Error> {
//     let entry = serde_json::json!({ "address": address, "index": index });
//     writeln!(writer, "{}", entry.to_string())?;
//     Ok(())
// }

pub(crate) fn save_address(
    address: &str,
    index: u32,
    tx: &std::sync::mpsc::Sender<(String, u32, String)>,
) -> Result<(), anyhow::Error> {
    let address_type = match address.starts_with("0x") {
        true => "eth",
        false => "tron",
    };

    tx.send((address.to_string(), index, address_type.to_string()))
        .map_err(|e| anyhow::anyhow!("Failed to send address: {}", e))?;

    Ok(())
}

pub(crate) fn write_last_index(filename: &str, index: u32) -> Result<(), anyhow::Error> {
    std::fs::write(filename, index.to_string())?;
    Ok(())
}

pub(crate) fn read_last_index(filename: &str) -> Result<u32, anyhow::Error> {
    let content = std::fs::read_to_string(filename).unwrap_or_else(|_| "0".to_string());
    let index: u32 = content.trim().parse().unwrap_or(0);
    Ok(index)
}

// #[derive(Serialize, Deserialize)]
// struct AddressRecord {
//     address: String,
//     index: u32,
// }

// pub(crate) fn save_address(address: &str, index: u32, filename: &str) -> Result<(), anyhow::Error> {
//     let new_record = AddressRecord {
//         address: address.to_string(),
//         index,
//     };

//     // 打开文件并读取现有记录
//     let file = OpenOptions::new()
//         .read(true)
//         .write(true)
//         .create(true)
//         .open(filename)?;
//     let reader = BufReader::new(&file);

//     // 如果文件为空，初始化一个空数组
//     let mut records: Vec<AddressRecord> = match serde_json::from_reader(reader) {
//         Ok(records) => records,
//         Err(_) => Vec::new(), // 如果文件为空或格式错误，初始化一个空的 Vec
//     };

//     // 追加新记录
//     records.push(new_record);

//     // 将所有记录写回文件
//     let file = std::fs::File::create(filename)?;
//     let writer = BufWriter::new(file);
//     serde_json::to_writer_pretty(writer, &records)?;

//     Ok(())
// }
