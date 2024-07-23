pub const ETH_DERIVATION_PATH: &str = "m/44'/60'/0'/0/0";
pub const TRON_DERIVATION_PATH: &str = "m/44'/195'/0'/0/0";

pub fn add_index(path: &str, index: u32) -> String {
    let parts: Vec<&str> = path.rsplitn(2, '/').collect();
    if parts.len() != 2 {
        panic!("Invalid derivation path");
    }
    format!("{}/{}", parts[1], index)
}