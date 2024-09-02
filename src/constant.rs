pub const ETH_DERIVATION_PATH: &str = "m/44'/60'/0'/0/0";
pub const ETH_HARD_DERIVATION_PATH: &str = "m/44h/60h/0h/0/0";

pub const TRON_DERIVATION_PATH: &str = "m/44'/195'/0'/0/0";
pub const TRON_HARD_DERIVATION_PATH: &str = "m/44h/195h/0h/0/0";

pub fn add_index(path: &str, index: u32, hard: bool) -> String {
    let parts: Vec<&str> = path.rsplitn(2, '/').collect();
    if parts.len() != 2 {
        panic!("Invalid derivation path");
    }
    let index = if hard {
        format!("{index}h")
    } else {
        format!("{index}")
    };
    format!("{}/{}", parts[1], index)
}
