#[derive(Clone, Copy)]
pub struct EthereumAddressGenerator;
impl super::AddressGenerator for EthereumAddressGenerator {
    fn generate_address(
        &self,
        key: &coins_bip32::xkeys::XPriv,
        index: isize,
    ) -> Result<String, anyhow::Error> {
        let path = if index < 0 {
            let i = index.strict_add_unsigned(coins_bip32::BIP32_HARDEN as usize) as u32;
            crate::constant::add_index(crate::constant::ETH_HARD_DERIVATION_PATH, i, true)
        } else {
            let i = index as u32;
            crate::constant::add_index(crate::constant::ETH_DERIVATION_PATH, i, false)
        };

        let derive = key.derive_path(path.as_str())?;
        // let derive = key.derive_path(path.as_str())?;
        let signingkey: &coins_bip32::ecdsa::SigningKey = derive.as_ref();

        let address = alloy::signers::utils::secret_key_to_address(signingkey);
        tracing::info!("address: {:?}, path: {}", address, path);
        Ok(address.to_string())
    }

    fn address_type(&self) -> String {
        "eth".to_string()
    }

    fn index_file_name(&self) -> String {
        "eth_last_index.txt".to_string()
    }
}
