use anychain_core::Address as _;

#[derive(Clone, Copy)]
pub struct TronAddressGenerator;

impl super::AddressGenerator for TronAddressGenerator {
    fn generate_address(
        &self,
        key: &coins_bip32::xkeys::XPriv,
        index: isize,
    ) -> Result<String, anyhow::Error> {
        let path = if index < 0 {
            let i = index.strict_add_unsigned(coins_bip32::BIP32_HARDEN as usize) as u32;
            crate::constant::add_index(crate::constant::TRON_HARD_DERIVATION_PATH, i, true)
        } else {
            let i = index as u32;
            crate::constant::add_index(crate::constant::TRON_DERIVATION_PATH, i, false)
        };
        let derive = key.derive_path(path.as_str())?;
        let signingkey: &coins_bip32::ecdsa::SigningKey = derive.as_ref();
        let private_key = signingkey.to_bytes();

        let private_key = libsecp256k1::SecretKey::parse_slice(&private_key)?;

        let address = anychain_tron::TronAddress::from_secret_key(
            &private_key,
            &anychain_tron::TronFormat::Standard,
        )
        .unwrap();
        tracing::info!("address: {:?}, path: {}", address, path);
        Ok(address.to_string())
    }

    fn address_type(&self) -> String {
        "tron".to_string()
    }

    fn index_file_name(&self) -> String {
        "tron_last_index.txt".to_string()
    }
}
